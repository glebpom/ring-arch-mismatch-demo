Steps to reproduce
==================

(works only under linux)

- Install [cross](https://github.com/rust-embedded/cross)
- Build without `ring`
```bash
cross build --target=arm-unknown-linux-gnueabi
```
- Check readelf output:
```bash
readelf -hA target/arm-unknown-linux-gnueabi/debug/ring-arch-mismatch-demo
```
Output:
```
File Attributes
  Tag_CPU_name: "ARM v6"
  Tag_CPU_arch: v6
  Tag_ARM_ISA_use: Yes
  Tag_THUMB_ISA_use: Thumb-1
```
- Build with `ring`
```bash
cross build --target=arm-unknown-linux-gnueabi --features=with-ring
```
- Check readelf output:
```bash
readelf -hA target/arm-unknown-linux-gnueabi/debug/ring-arch-mismatch-demo
```
Output:
```bash
File Attributes
  Tag_CPU_name: "7-A"
  Tag_CPU_arch: v7
  Tag_CPU_arch_profile: Application
  Tag_ARM_ISA_use: Yes
  Tag_THUMB_ISA_use: Thumb-2
  Tag_FP_arch: VFPv3
  Tag_Advanced_SIMD_arch: NEONv1
```
