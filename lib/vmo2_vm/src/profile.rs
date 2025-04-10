#[derive(Debug, Clone)]
pub struct Profile {
    pub executed_instructions: usize,
    pub total_memory_reads: usize,
    pub total_memory_writes: usize,
    pub total_stack_pushes: usize,
    pub total_stack_pops: usize,
}

impl Profile {
    pub fn new() -> Self {
        Self {
            executed_instructions: 0,
            total_memory_reads: 0,
            total_memory_writes: 0,
            total_stack_pushes: 0,
            total_stack_pops: 0,
        }
    }
}
