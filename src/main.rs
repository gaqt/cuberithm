use std::env;

pub mod cube;
pub mod side;

/*
 * args format:
 * colors for the initial state (54 total) (left->right, bottom->top, white on top, green on front)
 * colors: Y (yellow), B (blue), G (green), R (red), W (white), O (orange) or N (none/unspecified)
 * 'initial-state' then
 * first 9 chars: colors in white face
 * next 9 chars: colors in orange face
 * next 9 chars: colors in green face
 * next 9 chars: colors in red face
 * next 9 chars: colors in blue face
 * next 9 chars: colors in yellow face
 *
 * then 'desired-state' and repeat for desired state
 *
 * then 'include-m' to include middle moves
 * then 'include-w' to include wide moves
 * then 'fingertricks' to exclude bad fingertrick algorithms
 * then 'max-mem' followed by the maximum amount of memory used by in megabytes
 */
fn main() {
    let args: Vec<String> = env::args().collect();
}
