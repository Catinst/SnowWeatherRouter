#!/usr/bin/env python3
from __future__ import annotations
import argparse,hashlib,json,os,re,shutil,subprocess,tempfile,zipfile
from pathlib import Path
from axml_branding import extract_branding,patch_branding
from build_apk import apk_metadata,find_aapt2,find_android_jar,clone_info,SIGNATURE_RE

def run(cmd,capture=False):
 print('+',subprocess.list2cmdline([str(x) for x in cmd]));r=subprocess.run([str(x) for x in cmd],check=True,text=True,encoding='utf-8',errors='replace',capture_output=capture);return r.stdout if capture else ''

def manifest_xml(package,version_code,version_name):
 return f'''<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android" package="{package}" android:versionCode="{version_code}" android:versionName="{version_name}">
 <uses-sdk android:minSdkVersion="30" android:targetSdkVersion="36" />
 <uses-permission android:name="android.permission.INTERNET" />
 <uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />
 <uses-permission android:name="android.permission.ACCESS_WIFI_STATE" />
 <uses-permission android:name="android.permission.ACCESS_COARSE_LOCATION" />
 <uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />
 <uses-permission android:name="android.permission.VIBRATE" />
 <application android:label="@android:string/ok" android:icon="@android:drawable/ic_dialog_info" android:name="{package}.WeatherApplication" android:hasCode="true" android:hardwareAccelerated="true" android:extractNativeLibs="true" android:supportsRtl="true" android:directBootAware="true" android:defaultToDeviceProtectedStorage="true">
  <activity android:name="{package}.ActivityWeatherMain" android:theme="@android:style/Theme.Material.Light.NoActionBar" android:exported="true" android:launchMode="singleTask" android:screenOrientation="portrait" android:configChanges="orientation|keyboardHidden|screenSize|smallestScreenSize|screenLayout|density|uiMode|fontScale|fontWeightAdjustment" android:hardwareAccelerated="true">
   <meta-data android:name="android.app.lib_name" android:value="SnowWeatherRouter" />
   <intent-filter>
    <action android:name="android.intent.action.MAIN" /><category android:name="android.intent.category.LAUNCHER" />
   </intent-filter>
  </activity>
 </application>
</manifest>'''

def main():
 p=argparse.ArgumentParser();p.add_argument('input',type=Path);p.add_argument('--router',type=Path,required=True);p.add_argument('--dex',type=Path,required=True);p.add_argument('--output',type=Path,required=True);p.add_argument('--sdk',type=Path,required=True);p.add_argument('--source-package',default='com.miui.weather2');p.add_argument('--target-package',default='com.miui.weather3');p.add_argument('--version-increment',type=int,default=16);p.add_argument('--version-suffix',default='-Snow-v16-hybrid');a=p.parse_args()
 aapt=find_aapt2(a.sdk);android_jar=find_android_jar(a.sdk);source_pkg,vc,vn=apk_metadata(aapt,a.input.resolve())
 with zipfile.ZipFile(a.input) as z:branding=extract_branding(z.read('AndroidManifest.xml'))
 with tempfile.TemporaryDirectory(prefix='snow-hybrid-') as td:
  td=Path(td);m=td/'AndroidManifest.xml';m.write_text(manifest_xml(a.target_package,vc+a.version_increment,vn+a.version_suffix),encoding='utf-8');template=td/'template.apk';run([aapt,'link','--manifest',m,'-I',android_jar,'-o',template])
  with zipfile.ZipFile(template) as z:binary=z.read('AndroidManifest.xml')
  binary,brand=patch_branding(binary,branding,expected_launcher_activity=f"{a.target_package}.ActivityWeatherMain")
  olda=a.source_package.encode();newa=a.target_package.encode();oldu=a.source_package.encode('utf-16le');newu=a.target_package.encode('utf-16le');router=a.router.read_bytes();dex=a.dex.read_bytes();changed={};removed=[]
  with zipfile.ZipFile(a.input) as src,zipfile.ZipFile(a.output,'w',allowZip64=True) as dst:
   for info in src.infolist():
    if SIGNATURE_RE.match(info.filename):removed.append(info.filename);continue
    if info.filename in ('AndroidManifest.xml','classes.dex','lib/arm64-v8a/libSnowWeatherRouter.so'):continue
    data=src.read(info);ah=data.count(olda);uh=data.count(oldu);data=data.replace(olda,newa).replace(oldu,newu)
    if ah or uh:changed[info.filename]={'ascii_package_replacements':ah,'utf16_package_replacements':uh}
    c=clone_info(info)
    if info.compress_type==zipfile.ZIP_DEFLATED:dst.writestr(c,data,compress_type=info.compress_type,compresslevel=9)
    else:dst.writestr(c,data,compress_type=info.compress_type)
   for name,data,compress in [('AndroidManifest.xml',binary,zipfile.ZIP_DEFLATED),('classes.dex',dex,zipfile.ZIP_DEFLATED),('lib/arm64-v8a/libSnowWeatherRouter.so',router,zipfile.ZIP_STORED)]:
    zi=zipfile.ZipInfo(name);zi.compress_type=compress;zi.create_system=3;zi.external_attr=(0o100755 if name.endswith('.so') else 0o100644)<<16;dst.writestr(zi,data,compress_type=compress,compresslevel=9 if compress==zipfile.ZIP_DEFLATED else None);changed[name]={'hybrid':1}
 with zipfile.ZipFile(a.output) as z:assert z.testzip() is None;assert z.read('classes.dex')==dex;assert z.read('lib/arm64-v8a/libSnowWeatherRouter.so')==router
 print(json.dumps({'output':str(a.output.resolve()),'size':a.output.stat().st_size,'sha256':hashlib.sha256(a.output.read_bytes()).hexdigest(),'versionCode':vc+a.version_increment,'versionName':vn+a.version_suffix,'router_size':len(router),'router_sha256':hashlib.sha256(router).hexdigest(),'dex_size':len(dex),'dex_sha256':hashlib.sha256(dex).hexdigest(),'branding':brand,'removed_signatures':removed,'changed':changed},ensure_ascii=False,indent=2))
if __name__=='__main__':main()
