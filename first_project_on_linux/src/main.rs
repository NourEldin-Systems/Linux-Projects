struct Me{
    mind: Mind,
}

enum MindState{
    Busy,
    Free,
    Overwhelmed,
}

struct Mind {
    state: MindState,
}

fn main() {
    const MY_MIND: Mind = Mind {
        state: MindState::Busy,
    };

    const NOUR: Me = Me {
        mind: MY_MIND,
    };
}

