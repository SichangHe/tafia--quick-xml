use vstd::prelude::*;

verus! {

pub const OUTSIDE: u8 = 0;
pub const SINGLE_Q: u8 = 1;
pub const DOUBLE_Q: u8 = 2;
pub const GREATER: u8 = 62;
pub const SINGLE_QUOTE: u8 = 39;
pub const DOUBLE_QUOTE: u8 = 34;

pub struct Transition {
    pub state: u8,
    pub found: bool,
}

pub struct Boundary {
    pub state: u8,
    pub found: bool,
    pub relative_consumed: u8,
}

pub open spec fn found_spec(state: u8, byte: u8) -> bool {
    state == OUTSIDE && byte == GREATER
}

pub open spec fn state_spec(state: u8, byte: u8) -> u8 {
    if state == OUTSIDE && byte == SINGLE_QUOTE {
        SINGLE_Q
    } else if state == OUTSIDE && byte == DOUBLE_QUOTE {
        DOUBLE_Q
    } else if state == SINGLE_Q && byte == SINGLE_QUOTE {
        OUTSIDE
    } else if state == DOUBLE_Q && byte == DOUBLE_QUOTE {
        OUTSIDE
    } else {
        state
    }
}

fn step(state: u8, byte: u8) -> (result: Transition)
    requires
        state <= DOUBLE_Q,
    ensures
        result.found == found_spec(state, byte),
        result.state == state_spec(state, byte),
        result.state <= DOUBLE_Q,
{
    if state == OUTSIDE && byte == GREATER {
        Transition { state, found: true }
    } else if state == OUTSIDE && byte == SINGLE_QUOTE {
        Transition { state: SINGLE_Q, found: false }
    } else if state == OUTSIDE && byte == DOUBLE_QUOTE {
        Transition { state: DOUBLE_Q, found: false }
    } else if state == SINGLE_Q && byte == SINGLE_QUOTE {
        Transition { state: OUTSIDE, found: false }
    } else if state == DOUBLE_Q && byte == DOUBLE_QUOTE {
        Transition { state: OUTSIDE, found: false }
    } else {
        Transition { state, found: false }
    }
}

fn scan_payload(
    len: u8,
    byte0: u8,
    byte1: u8,
    byte2: u8,
    byte3: u8,
    byte4: u8,
    byte5: u8,
) -> (result: Transition)
    requires
        len <= 6,
        byte0 != SINGLE_QUOTE,
        byte1 != SINGLE_QUOTE,
        byte2 != SINGLE_QUOTE,
        byte3 != SINGLE_QUOTE,
        byte4 != SINGLE_QUOTE,
        byte5 != SINGLE_QUOTE,
    ensures
        result.state == SINGLE_Q,
        !result.found,
{
    let mut state = SINGLE_Q;
    if len >= 1 {
        let transition = step(state, byte0);
        assert(transition.state == SINGLE_Q);
        assert(!transition.found);
        state = transition.state;
    }
    if len >= 2 {
        let transition = step(state, byte1);
        assert(transition.state == SINGLE_Q);
        assert(!transition.found);
        state = transition.state;
    }
    if len >= 3 {
        let transition = step(state, byte2);
        assert(transition.state == SINGLE_Q);
        assert(!transition.found);
        state = transition.state;
    }
    if len >= 4 {
        let transition = step(state, byte3);
        assert(transition.state == SINGLE_Q);
        assert(!transition.found);
        state = transition.state;
    }
    if len >= 5 {
        let transition = step(state, byte4);
        assert(transition.state == SINGLE_Q);
        assert(!transition.found);
        state = transition.state;
    }
    if len >= 6 {
        let transition = step(state, byte5);
        assert(transition.state == SINGLE_Q);
        assert(!transition.found);
        state = transition.state;
    }
    Transition { state, found: false }
}

fn scan_single_quoted_boundary(
    len: u8,
    byte0: u8,
    byte1: u8,
    byte2: u8,
    byte3: u8,
    byte4: u8,
    byte5: u8,
) -> (result: Boundary)
    requires
        len <= 6,
        byte0 != SINGLE_QUOTE,
        byte1 != SINGLE_QUOTE,
        byte2 != SINGLE_QUOTE,
        byte3 != SINGLE_QUOTE,
        byte4 != SINGLE_QUOTE,
        byte5 != SINGLE_QUOTE,
    ensures
        result.state == OUTSIDE,
        result.found,
        result.relative_consumed == len + 2,
        result.relative_consumed <= 8,
{
    let opening = step(OUTSIDE, SINGLE_QUOTE);
    assert(opening.state == SINGLE_Q);
    assert(!opening.found);

    let payload = scan_payload(len, byte0, byte1, byte2, byte3, byte4, byte5);
    assert(payload.state == SINGLE_Q);
    assert(!payload.found);

    let closing = step(payload.state, SINGLE_QUOTE);
    assert(closing.state == OUTSIDE);
    assert(!closing.found);

    let boundary = step(closing.state, GREATER);
    assert(boundary.state == OUTSIDE);
    assert(boundary.found);

    Boundary {
        state: boundary.state,
        found: boundary.found,
        relative_consumed: len + 2,
    }
}

pub open spec fn eof_kind_spec(state: u8) -> u8 {
    if state == SINGLE_Q {
        1
    } else if state == DOUBLE_Q {
        2
    } else {
        0
    }
}

fn eof_kind(state: u8) -> (result: u8)
    requires
        state <= DOUBLE_Q,
    ensures
        result == eof_kind_spec(state),
{
    if state == SINGLE_Q {
        1
    } else if state == DOUBLE_Q {
        2
    } else {
        0
    }
}

fn main() {
    let boundary = scan_single_quoted_boundary(3, GREATER, DOUBLE_QUOTE, 97, 0, 0, 0);
    assert(boundary.state == OUTSIDE);
    assert(boundary.found);
    assert(boundary.relative_consumed == 5);

    let unclosed = scan_payload(3, GREATER, DOUBLE_QUOTE, 97, 0, 0, 0);
    let unclosed_kind = eof_kind(unclosed.state);
    assert(unclosed_kind == 1);

    let closed = step(unclosed.state, SINGLE_QUOTE);
    assert(closed.state == OUTSIDE);
    let closed_kind = eof_kind(closed.state);
    assert(closed_kind == 0);
}

}
