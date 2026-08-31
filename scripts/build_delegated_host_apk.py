#!/usr/bin/env python3
from __future__ import annotations
import argparse,hashlib,json,re,shutil,struct,subprocess,sys,tempfile,zipfile
from pathlib import Path
from axml_branding import parse_events,StartElement,ANDROID_NAME,TYPE_STRING,u16,u32,p32,NO_INDEX

SIGNATURE_RE=re.compile(r'^META-INF/(?:MANIFEST\.MF|[^/]+\.(?:SF|RSA|DSA|EC))$',re.I)
ANDROID_VERSION_CODE=0x0101021B
ANDROID_VERSION_NAME=0x0101021C
RES_XML_START_ELEMENT_TYPE=0x0102
RES_XML_END_ELEMENT_TYPE=0x0103
ROUTER_ALIAS='libSnowWeatherRouter.so'
ORIGINAL_HOST='libweather_app.so'
MANIFEST_STRING_REPLACEMENTS={
 'weather;weatherinfo':'weathr3;weathr3info',
 'weather.location':'weathr3.location',
}
BINARY_REPLACEMENTS=[
 (b'weather;weatherinfo',b'weathr3;weathr3info'),
 (b'weather.location',b'weathr3.location'),
 (b'content://weather',b'content://weathr3'),
]

def run(cmd):
 print('+',subprocess.list2cmdline([str(x) for x in cmd]));subprocess.run([str(x) for x in cmd],check=True)

def read_length16(data,offset):
 v=u16(data,offset);offset+=2
 if v&0x8000:v=((v&0x7fff)<<16)|u16(data,offset);offset+=2
 return v,offset

def remove_required_rust_runtime_library(data: bytes) -> tuple[bytes, int]:
 out=bytearray(data);pos=8;strings=[];remove=[];hits=0
 while pos<len(out):
  typ=u16(out,pos);hs=u16(out,pos+2);size=u32(out,pos+4)
  if typ==1:
   count=u32(out,pos+8);flags=u32(out,pos+16);start=u32(out,pos+20);utf8=bool(flags&0x100);offs=[u32(out,pos+hs+i*4) for i in range(count)]
   strings=[]
   for rel in offs:
    p=pos+start+rel
    if utf8:
     def r8(q):
      v=out[q];q+=1
      if v&0x80:v=((v&0x7f)<<8)|out[q];q+=1
      return v,q
     _,p=r8(p);n,p=r8(p);strings.append(bytes(out[p:p+n]).decode('utf-8','replace'))
    else:
     n,p=read_length16(out,p);strings.append(bytes(out[p:p+n*2]).decode('utf-16le','replace'))
  elif typ==RES_XML_START_ELEMENT_TYPE and strings:
   ext=pos+hs;name_idx=u32(out,ext+4);name=strings[name_idx];attr_start=u16(out,ext+8);attr_size=u16(out,ext+10);attr_count=u16(out,ext+12);attrs=ext+attr_start
   if name=='uses-library':
    value=None
    for i in range(attr_count):
     ao=attrs+i*attr_size;raw=u32(out,ao+8)
     if raw!=NO_INDEX and raw<len(strings):
      s=strings[raw]
      if s=='hyperos.rustruntime.v3':value=s
    if value:
     start_pos=pos;depth=1;cursor=pos+size
     while cursor<len(out) and depth:
      ct=u16(out,cursor);cs=u32(out,cursor+4)
      if ct==RES_XML_START_ELEMENT_TYPE:depth+=1
      elif ct==RES_XML_END_ELEMENT_TYPE:depth-=1
      cursor+=cs
     remove.append((start_pos,cursor));hits+=1
  pos+=size
 if hits!=1:raise ValueError(f'expected one hyperos runtime uses-library node, got {hits}')
 for a,b in reversed(remove):del out[a:b]
 p32(out,4,len(out))
 return bytes(out),hits

def encode_length16(value: int) -> bytes:
 if value < 0x8000:return struct.pack('<H',value)
 return struct.pack('<HH',0x8000 | (value >> 16),value & 0xffff)

def rebuild_utf16_string_pool(data: bytes, replacements: dict[str,str]) -> tuple[bytes,dict[str,int]]:
 out=bytearray(data);pos=8;pool=None
 while pos<len(out):
  typ=u16(out,pos);size=u32(out,pos+4)
  if typ==1:pool=pos;break
  pos+=size
 if pool is None:raise ValueError('string pool missing')
 hs=u16(out,pool+2);old_size=u32(out,pool+4);count=u32(out,pool+8);styles=u32(out,pool+12);flags=u32(out,pool+16)
 if flags&0x100:raise ValueError('expected UTF-16 manifest pool')
 if styles:raise ValueError('styled string pools are not supported')
 strings_start=u32(out,pool+20);values=[];counts={key:0 for key in replacements}
 for i in range(count):
  rel=u32(out,pool+hs+i*4);p=pool+strings_start+rel;n,q=read_length16(out,p);s=bytes(out[q:q+n*2]).decode('utf-16le','replace')
  if s in replacements:counts[s]+=1;s=replacements[s]
  values.append(s)
 payload=bytearray();offsets=[]
 for s in values:
  offsets.append(len(payload));encoded=s.encode('utf-16le');payload+=encode_length16(len(s))+encoded+b'\0\0'
 while len(payload)%4:payload.append(0)
 new_strings_start=hs+count*4
 chunk=bytearray(out[pool:pool+hs]);chunk+=b'\0'*(count*4)
 for i,rel in enumerate(offsets):p32(chunk,hs+i*4,rel)
 chunk+=payload;p32(chunk,4,len(chunk));p32(chunk,20,new_strings_start);p32(chunk,24,0)
 out[pool:pool+old_size]=chunk;p32(out,4,len(out))
 return bytes(out),counts

def patch_manifest(data:bytes,source_package:str,target_package:str,version_code:int,version_name:str):
 data,runtime_nodes_removed=remove_required_rust_runtime_library(data)
 replacements={ORIGINAL_HOST:ROUTER_ALIAS,**MANIFEST_STRING_REPLACEMENTS}
 data,string_hits=rebuild_utf16_string_pool(data,replacements)
 out=bytearray(data);old=source_package.encode('utf-16le');new=target_package.encode('utf-16le')
 if len(old)!=len(new):raise ValueError('package length mismatch')
 package_hits=out.count(old);out[:]=out.replace(old,new)
 host_hits=string_hits[ORIGINAL_HOST];version_name_hits=0
 authority_hits={key:string_hits[key] for key in MANIFEST_STRING_REPLACEMENTS}
 # patch versionCode typed integer in manifest element
 version_code_hits=0
 for ev in parse_events(bytes(out)):
  if isinstance(ev,StartElement) and ev.name=='manifest':
   a=ev.attribute(ANDROID_VERSION_CODE)
   if a:p32(out,a.data_offset,version_code);version_code_hits+=1
 if host_hits!=1:raise ValueError(f'expected one app lib name, got {host_hits}')
 if version_code_hits!=1:raise ValueError(f'versionCode hits {version_code_hits}')
 return bytes(out),{'package_hits':package_hits,'host_alias_hits':host_hits,'version_code_hits':version_code_hits,'version_name_hits':version_name_hits,'runtime_nodes_removed':runtime_nodes_removed,'authority_hits':authority_hits}

def main():
 ap=argparse.ArgumentParser();ap.add_argument('input',type=Path);ap.add_argument('--router',type=Path,required=True);ap.add_argument('--output',type=Path,required=True);ap.add_argument('--source-package',default='com.miui.weather2');ap.add_argument('--target-package',default='com.miui.weather3');args=ap.parse_args()
 with zipfile.ZipFile(args.input) as src:
  manifest,report=patch_manifest(src.read('AndroidManifest.xml'),args.source_package,args.target_package,180000244,'[IP]-R')
  router=args.router.read_bytes();args.output.parent.mkdir(parents=True,exist_ok=True)
  with zipfile.ZipFile(args.output,'w',allowZip64=True) as dst:
   names=[]
   for info in src.infolist():
    names.append(info.filename)
    if SIGNATURE_RE.match(info.filename):continue
    data=manifest if info.filename=='AndroidManifest.xml' else src.read(info)
    olda=args.source_package.encode();newa=args.target_package.encode();oldu=args.source_package.encode('utf-16le');newu=args.target_package.encode('utf-16le')
    if info.filename!='AndroidManifest.xml':
     data=data.replace(olda,newa).replace(oldu,newu)
     for old_value,new_value in BINARY_REPLACEMENTS:
      if len(old_value)!=len(new_value):raise ValueError((old_value,new_value))
      data=data.replace(old_value,new_value).replace(old_value.decode().encode('utf-16le'),new_value.decode().encode('utf-16le'))
    clone=zipfile.ZipInfo(info.filename,info.date_time);clone.compress_type=info.compress_type;clone.create_system=info.create_system;clone.external_attr=info.external_attr;clone.extra=info.extra;clone.comment=info.comment
    if info.compress_type==zipfile.ZIP_DEFLATED:dst.writestr(clone,data,compress_type=info.compress_type,compresslevel=9)
    else:dst.writestr(clone,data,compress_type=info.compress_type)
   for name in ['lib/arm64-v8a/'+ROUTER_ALIAS]:
    zi=zipfile.ZipInfo(name);zi.compress_type=zipfile.ZIP_STORED;zi.create_system=3;zi.external_attr=0o100755<<16;dst.writestr(zi,router,compress_type=zipfile.ZIP_STORED)
 with zipfile.ZipFile(args.output) as z:
  bad=z.testzip();assert bad is None;assert z.read('lib/arm64-v8a/'+ROUTER_ALIAS)==router;assert z.read('lib/arm64-v8a/libweather_app.so')
 print(json.dumps({'input':str(args.input.resolve()),'output':str(args.output.resolve()),'size':args.output.stat().st_size,'sha256':hashlib.sha256(args.output.read_bytes()).hexdigest(),'router_size':len(router),'router_sha256':hashlib.sha256(router).hexdigest(),'manifest':report,'alias':ROUTER_ALIAS},ensure_ascii=False,indent=2))
if __name__=='__main__':main()
