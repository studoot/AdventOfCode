pub fn parse(input: &str) -> Vec<usize> {
    input.lines().flat_map(|s| s.parse::<usize>()).collect()
}

fn main() {
    let input_string = include_str!("../input.txt");
    let measurements = parse(input_string);

    {
        let count_larger = measurements.windows(2).filter(|s| s[1] > s[0]).count();
        println!("Day  1 part 1 - depth increase count = {}", count_larger);
    }
    {
        // Rolling average = m[i,i+1,i+2] where the oldest point i. So - at time
        // t, the current rolling average is `m[t-2]+m[t-1]+m[t]` and the
        // previous one is `m[t-3]+m[t-2]+m[t-1]`.
        //
        // We want to see if `current>previous`, i.e. `(m[t-2]+m[t-1]+m[t]) >
        // (m[t-3]+m[t-2]+m[t-1])`. The terms `m[t-2]` and `m[t-1]` can be
        // eliminated from both sides of this, so we just need to see if `m[t] >
        // m[t-3]`.
        let count_larger = measurements.windows(4).filter(|s| s[3] > s[0]).count();
        println!("Day  1 part 2 - rolling average depth increase count = {}", count_larger);
    }
}
