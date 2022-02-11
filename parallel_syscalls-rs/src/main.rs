use std::{ptr::null_mut, intrinsics::transmute};
use ntapi::ntpsapi::PPS_ATTRIBUTE_LIST;
use winapi::{um::{memoryapi::VirtualAlloc, winnt::{ACCESS_MASK, GENERIC_ALL, MEM_RESERVE, MEM_COMMIT, PAGE_EXECUTE_READWRITE}, processthreadsapi::GetCurrentProcess}, shared::{ntdef::{PHANDLE, POBJECT_ATTRIBUTES, HANDLE, PVOID, NTSTATUS, NT_SUCCESS}, minwindef::ULONG, basetsd::SIZE_T}, ctypes::c_void};

mod parallel_syscalls;

// Function to call
type NtCreateThreadEx = unsafe extern "system" fn(
    ThreadHandle: PHANDLE, 
    DesiredAccess: ACCESS_MASK, 
    ObjectAttributes: POBJECT_ATTRIBUTES, 
    ProcessHandle: HANDLE, 
    StartRoutine: PVOID, 
    Argument: PVOID, 
    CreateFlags: ULONG, 
    ZeroBits: SIZE_T, 
    StackSize: SIZE_T, 
    MaximumStackSize: SIZE_T, 
    AttributeList: PPS_ATTRIBUTE_LIST
) -> NTSTATUS;

const MAX_SYSCALL_STUB_SIZE: u32 = 64;

fn main() {

    // Dynamically get the base address of a fresh copy of ntdll.dll using mdsec's technique
    let ptr_ntdll = parallel_syscalls::gimme_the_loot("ntdll");

    if ptr_ntdll.is_null() {
        panic!("Pointer to ntdll is null");
    }

    //get function address
    let syscall_nt_create_thread_ex = parallel_syscalls::get_function_address(ptr_ntdll, "NtCreateThreadEx");

    // Allocate memory for the system call (not optimal from opsec perspective)
    let syscall_region = unsafe { VirtualAlloc(null_mut(), MAX_SYSCALL_STUB_SIZE as usize, MEM_RESERVE | MEM_COMMIT, PAGE_EXECUTE_READWRITE) as usize };

    if syscall_region == 0 {
        panic!("Failed to allocate memory using VirtualAlloc in main");
    }


    let nt_create_thread_ex = unsafe { parallel_syscalls::build_syscall_stub(syscall_region as *mut c_void, syscall_nt_create_thread_ex as u32) };
    
    // Example
    unsafe {
        let syscall_nt_create_thread_ex = transmute::<_, NtCreateThreadEx>(nt_create_thread_ex);
        let mut thread_handle : *mut c_void = null_mut();

        let status = syscall_nt_create_thread_ex(&mut thread_handle, GENERIC_ALL, null_mut(), GetCurrentProcess(), null_mut(), null_mut(), 0, 0, 0, 0, null_mut());

        if !NT_SUCCESS(status) {
            panic!("Failed to call NtCreateThreadEx");
        }

        println!("[+] Thread Handle: {:?} and Status: {:?}", thread_handle, status);
    }
}