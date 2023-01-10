use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::um::memoryapi;
use winapi::um::processthreadsapi;
use winapi::um::sysinfoapi::{self, LPSYSTEM_INFO, SYSTEM_INFO};
use winapi::um::winnt::{HANDLE, MEMORY_BASIC_INFORMATION, PVOID};

fn main() {
    let this_pid: DWORD;
    let this_proc: HANDLE;
    let min_addr: LPVOID;
    let max_addr: LPVOID;
    let mut base_addr: PVOID;
    let mut proc_info: SYSTEM_INFO;
    let mut mem_info: MEMORY_BASIC_INFORMATION;

    const MEMINFO_SIZE: usize = std::mem::size_of::<MEMORY_BASIC_INFORMATION>();
    println!("{MEMINFO_SIZE}");

    unsafe {
        base_addr = std::mem::zeroed();
        proc_info = std::mem::zeroed();
        mem_info = std::mem::zeroed();
    }

    unsafe {
        this_pid = processthreadsapi::GetCurrentProcessId();
        this_proc = processthreadsapi::GetCurrentProcess();
        sysinfoapi::GetSystemInfo(&mut proc_info as LPSYSTEM_INFO);
    };

    min_addr = proc_info.lpMinimumApplicationAddress;
    max_addr = proc_info.lpMaximumApplicationAddress;

    println!("{:?} @ {:p}", this_pid, this_proc);
    println!("Размер страницы: {:?}", proc_info.dwPageSize);
    println!(
        "Количество процессоров в системе: {:?}",
        proc_info.dwNumberOfProcessors
    );
    println!("Указатель на младший адрес памяти: {:p}", min_addr);
    println!("Указатель на старший адрес памяти: {:p}", max_addr);

    loop {
        let rc: SIZE_T = unsafe {
            memoryapi::VirtualQueryEx(this_proc, base_addr, &mut mem_info, MEMINFO_SIZE as SIZE_T)
        };

        if rc == 0 {
            break;
        }

        println!("MEMORY_BASIC_INFORMATION {{");
        println!("\tBaseAddress: {:p}", mem_info.BaseAddress);
        println!("\tAllocationBase: {:p}", mem_info.AllocationBase);
        println!("\tAllocationProtect: {}", mem_info.AllocationProtect);
        println!("\tRegionSize: {}", mem_info.RegionSize);
        println!("\tState: {}", mem_info.State);
        println!("\tProtect: {}", mem_info.Protect);
        println!("\tType: {}", mem_info.Type);
        println!("}}");

        base_addr = ((base_addr as u64) + (mem_info.RegionSize as u64)) as PVOID;
    }
}
