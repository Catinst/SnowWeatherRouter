#!/usr/bin/env python3
from __future__ import annotations
import argparse,subprocess
from pathlib import Path

def run(cmd):
 print('+',subprocess.list2cmdline([str(x) for x in cmd]));subprocess.run([str(x) for x in cmd],check=True)

def main():
 p=argparse.ArgumentParser();p.add_argument('--rustc',required=True);p.add_argument('--output',type=Path,required=True);a=p.parse_args();repo=Path(__file__).resolve().parents[1];stubs=repo/'.tools/link-stubs';stubs.mkdir(parents=True,exist_ok=True)
 for source,name,soname in [(repo/'router/link_stubs/liblog.rs','liblog.so','liblog.so'),(repo/'router/link_stubs/libdl.rs','libdl.so','libdl.so'),(repo/'router/link_stubs/libc.rs','libc.so','libc.so')]:
  run([a.rustc,'--edition=2021','--target','aarch64-linux-android','--crate-type','cdylib','-C','panic=abort','-C','linker=rust-lld','-C','link-arg=--build-id=none','-C',f'link-arg=-soname={soname}',str(source),'-o',str(stubs/name)])
 a.output.parent.mkdir(parents=True,exist_ok=True)
 run([a.rustc,'--edition=2021','--target','aarch64-linux-android','--crate-type','cdylib','-C','panic=abort','-C','opt-level=z','-C','lto=fat','-C','codegen-units=1','-C','strip=symbols','-C','linker=rust-lld','-C','link-arg=--build-id=sha1','-C','link-arg=-soname=libSnowWeatherRouter.so','-C','link-arg=--no-as-needed','-C',f'link-arg=-L{stubs}','-C','link-arg=-l:liblog.so','-C','link-arg=-l:libdl.so','-C','link-arg=-l:libc.so',str(repo/'router/src/delegate.rs'),'-o',str(a.output)])
if __name__=='__main__':main()
