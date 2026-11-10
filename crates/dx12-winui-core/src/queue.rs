use crate::errors::Result;
use windows::Win32::Graphics::Direct3D12::*;

pub struct CommandQueues {
    pub direct_queue: ID3D12CommandQueue,
    pub compute_queue: ID3D12CommandQueue,
    pub copy_queue: ID3D12CommandQueue,
}

impl CommandQueues {
    pub fn new(device: &ID3D12Device) -> Result<Self> {
        unsafe {
            let direct_desc = D3D12_COMMAND_QUEUE_DESC {
                Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
                Priority: D3D12_COMMAND_QUEUE_PRIORITY_HIGH.0,
                Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
                NodeMask: 0,
            };
            let direct_queue: ID3D12CommandQueue = device.CreateCommandQueue(&direct_desc)?;

            let compute_desc = D3D12_COMMAND_QUEUE_DESC {
                Type: D3D12_COMMAND_LIST_TYPE_COMPUTE,
                Priority: D3D12_COMMAND_QUEUE_PRIORITY_NORMAL.0,
                Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
                NodeMask: 0,
            };
            let compute_queue: ID3D12CommandQueue = device.CreateCommandQueue(&compute_desc)?;

            let copy_desc = D3D12_COMMAND_QUEUE_DESC {
                Type: D3D12_COMMAND_LIST_TYPE_COPY,
                Priority: D3D12_COMMAND_QUEUE_PRIORITY_NORMAL.0,
                Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
                NodeMask: 0,
            };
            let copy_queue: ID3D12CommandQueue = device.CreateCommandQueue(&copy_desc)?;

            Ok(Self {
                direct_queue,
                compute_queue,
                copy_queue,
            })
        }
    }
}

pub struct DescriptorHeaps {
    pub rtv_heap: ID3D12DescriptorHeap,
    pub srv_heap: ID3D12DescriptorHeap,
    pub rtv_descriptor_size: u32,
    pub srv_descriptor_size: u32,
}

impl DescriptorHeaps {
    pub fn new(device: &ID3D12Device, num_rtv: u32, num_srv: u32) -> Result<Self> {
        unsafe {
            let rtv_desc = D3D12_DESCRIPTOR_HEAP_DESC {
                Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
                NumDescriptors: num_rtv,
                Flags: D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
                NodeMask: 0,
            };
            let rtv_heap: ID3D12DescriptorHeap = device.CreateDescriptorHeap(&rtv_desc)?;

            let srv_desc = D3D12_DESCRIPTOR_HEAP_DESC {
                Type: D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
                NumDescriptors: num_srv,
                Flags: D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE,
                NodeMask: 0,
            };
            let srv_heap: ID3D12DescriptorHeap = device.CreateDescriptorHeap(&srv_desc)?;

            let rtv_descriptor_size =
                device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV);
            let srv_descriptor_size =
                device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV);

            Ok(Self {
                rtv_heap,
                srv_heap,
                rtv_descriptor_size,
                srv_descriptor_size,
            })
        }
    }
}
