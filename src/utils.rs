use substreams_aelf_core::pb::aelf::*;

enum TraceGroup {
    Pre,
    Self_,
    Inline,
    Post,
}

pub struct TransactionTractStateIterator<'a> {
    current_trace_group: TraceGroup,
    current_index: usize,
    is_successful: bool,
    this_trace: &'a TransactionTrace,
    pre_traces_iters: Vec<TransactionTractStateIterator<'a>>,
    inline_traces_iters: Vec<TransactionTractStateIterator<'a>>,
    post_traces_iters: Vec<TransactionTractStateIterator<'a>>,
}

fn is_successful<'a>(trace: &'a TransactionTrace) -> bool {
    if trace.execution_status != substreams_aelf_core::pb::aelf::ExecutionStatus::Executed.into() { return false; }
    if trace.pre_traces.iter().any(|trace| !is_successful(trace)) { return false; }
    if trace.inline_traces.iter().any(|trace| !is_successful(trace)) { return false; }
    if trace.post_traces.iter().any(|trace| !is_successful(trace)) { return false; }
    return true;
}

impl<'a> TransactionTractStateIterator<'a> {
    pub fn new(trace: &'a TransactionTrace) -> Self {
        let successful = is_successful(trace);
        let pre_traces_iters = trace.pre_traces.iter().filter(|trace| is_successful(*trace)).map(|t| Self::new(t)).collect();
        let inline_traces_iters = if successful {
            trace.inline_traces.iter().map(|t| Self::new(t)).collect()
        } else {
            vec![]
        };
        let post_traces_iters = trace.post_traces.iter().filter(|trace| is_successful(*trace)).map(|t| Self::new(t)).collect();
        Self {
            current_trace_group: TraceGroup::Pre,
            current_index: 0,
            is_successful: successful,
            this_trace: trace,
            pre_traces_iters,
            inline_traces_iters,
            post_traces_iters,
        }
    }


    fn next_pre(&mut self) -> Option<<TransactionTractStateIterator<'a> as Iterator>::Item> {
        if self.current_index < self.pre_traces_iters.len() {
            self.pre_traces_iters[self.current_index].next()
        } else {
            None
        }
    }
    fn next_inline(&mut self) -> Option<<TransactionTractStateIterator<'a> as Iterator>::Item> {
        if self.current_index < self.inline_traces_iters.len() {
            self.inline_traces_iters[self.current_index].next()
        } else {
            None
        }
    }
    fn next_post(&mut self) -> Option<<TransactionTractStateIterator<'a> as Iterator>::Item> {
        if self.current_index < self.post_traces_iters.len() {
            self.post_traces_iters[self.current_index].next()
        } else {
            None
        }
    }
}

impl<'a> Iterator for TransactionTractStateIterator<'a> {
    type Item = &'a TransactionTrace;


    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.current_trace_group {
                TraceGroup::Pre => {
                    if let Some(trace) = self.next_pre() {
                        return Some(trace);
                    } else {
                        self.current_trace_group = TraceGroup::Self_;
                        self.current_index = 0;
                    }
                }
                TraceGroup::Self_ => {
                    self.current_trace_group = TraceGroup::Inline;
                    self.current_index = 0;
                    return Some(self.this_trace);
                }
                TraceGroup::Inline => {
                    if let Some(trace) = self.next_inline() {
                        self.current_index += 1;
                        return Some(trace);
                    }
                    self.current_trace_group = TraceGroup::Post;
                    self.current_index = 0;
                }
                TraceGroup::Post => {
                    return self.next_post();
                }
            }
        }
    }
}