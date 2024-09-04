# Ordinal System ( OS for short :P ) - Rust-based OS (hobby)

## About
This is a project I undertook to learn the fundamentals of OS development along with learning the Rust programming language.
So far the goals of this project haven't been decided and will be subject to change in the future

## Progress
+ Jumped to long (64-bit) mode directly from real mode
+ Setup of GDT and page tables have been completed  [Note: ID mapped first 2 MiB]
+ kernel with basic (overstatement in my opinion) functionality has been implemented in the Rust programming language.
+ Writing a basic ATA interface to load the complete kernel

## Requirements
If you want to tweak this project, please make sure to have Qemu, Make, Rust nightly build, llvm clang and ld.

## Usage
Clone the repo, I suggest you `cargo clean` and `make clean` before you run anything. Once done you should be safe to `make run`, this will boot the OS in Qemu.
make sure to pay attention to the flags provided when using Qemu (refer to Qemu docs for more details on this)


<h4>I think I have commented on the code and provided the references appropriately. If I have missed anything, please let me know. Thank you!</h4>

## Cool ScreenShot
![welcome](https://github.com/kushurox/OS/assets/38589424/82fea91b-d527-49a2-a4ce-c1112fc30767)
