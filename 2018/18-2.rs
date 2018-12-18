use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

const NEW_LINE : u8 = 10;

/*
    Read a file given by a filename and parse it to a vector of
    whatever we need given a line_transform function
*/
fn get_input<T>(filename : &str, line_transform : fn(String) -> T) -> Vec<T> {
    let mut file = File::open(filename).expect("Cannot open file");
    let mut ret : Vec<T> = Vec::new();
    let mut vec_one_line = Vec::new();
    loop {
        let mut buf = [0;512];
        let res = file.read(&mut buf);
        match res {
            Ok(n) => {
                if n == 0 {
                    // If the file doesn't end on a new line, we'll have stuff
                    // in our "current line" vector, thus we need to parse it as well
                    if vec_one_line.len() > 0 {
                        let s = String::from_utf8(vec_one_line.clone()).unwrap();
                        ret.push(line_transform(s));
                    }
                    break;
                }

                for b in buf.iter() {
                    if *b == NEW_LINE {
                        let s = String::from_utf8(vec_one_line.clone()).unwrap();
                        vec_one_line.clear();
                        ret.push(line_transform(s));
                    } else {
                        // deal with null bytes, that are in the buffer ('cause it's fixed size), when
                        // we're at the end of the file. Since we're reading a text file those shouldn't be in
                        // legitimate places, so they're safe to ignore
                        if *b > 0 {
                            vec_one_line.push(*b);
                        }
                    }
                }
            }
            _ => {
                break;
            }
        }
    }
    return ret;

}

fn main () {
    let input = get_input("input-18", |s| s );

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut maxy = 0usize;
    let mut maxx = 0usize;

    for line in input {
        maxx = line.len();
        maxy+=1;
        let l : Vec<char> = line.chars().collect();
        grid.push(l);
    }

    let mut cycle_start = 0usize;
    let mut last_seen_at : HashMap<(u16,u16),usize> = HashMap::new();
    let mut at_end_of_each : Vec<(u16,u16)> = Vec::new();
    
    let mut cycle_size = 0;

    let mut maybe_a_loop = false;
    

    for i in 0..10000 {
        
        let mut newgrid : Vec<Vec<char>> = Vec::new();
        for y in 0..maxy {
            let mut newline : Vec<char> = Vec::new();
            for x in 0..maxx {
                let oldstate = grid[y][x];
                let x_start = if x > 0  { x - 1 } else { 0 };
                let x_max = if x + 1 < maxx { x + 2 } else { maxx };
                let y_start = if y > 0 { y - 1 } else { 0 };
                let y_max = if y + 1 < maxy { y + 2 } else { maxy };
                let mut cnt_lumber = 0;
                let mut cnt_trees = 0;

                for yy in y_start..y_max {
                    for xx in x_start..x_max {
                        if xx == x && yy == y { continue; }
                        if grid[yy][xx] == '|' {
                            cnt_trees += 1;
                        } else if grid[yy][xx] == '#' {
                            cnt_lumber += 1;
                        }
                    }
                }
                //println!("Y,X: {} {}, old: {}, adj l {}, adj tree {} ", y,x,oldstate, cnt_lumber,cnt_trees);
                if oldstate == '.' {
                    newline.push(
                        if cnt_trees >= 3 { '|' } else { '.' }
                    );
                } else if oldstate == '|' {
                    newline.push(
                        if cnt_lumber >= 3 { '#' } else { '|' }
                    );
                } else {
                    newline.push(
                        if cnt_lumber > 0 && cnt_trees > 0 { '#' } else { '.' }
                    );
                }
            }
            newgrid.push(newline);
        }
        grid = newgrid;
        let mut cnt_lumber = 0;
        let mut cnt_trees = 0;
        for y in 0..maxy {
            for x in 0..maxx {
                if grid[y][x] == '|' {
                    cnt_trees += 1;
                } else if grid[y][x] == '#' {
                    cnt_lumber += 1;
                }
            }
        }

        at_end_of_each.push((cnt_lumber,cnt_trees));
        if !maybe_a_loop {
            if last_seen_at.contains_key(&(cnt_lumber,cnt_trees)) {
                cycle_start = *last_seen_at.get(&(cnt_lumber,cnt_trees)).unwrap();
                cycle_size = i - cycle_start;
                maybe_a_loop = true;
            }
        } else {
            let (then_lum, then_tre) = at_end_of_each[i-cycle_size];
            if then_lum != cnt_lumber || then_tre != cnt_trees {
                maybe_a_loop = false;
            } else {
                if i - 2*cycle_size == cycle_start {
                    break;
                }
            }
        }
        last_seen_at.insert((cnt_lumber,cnt_trees),i);

    }

    let offset = (1000000000 - 1 - cycle_start) % cycle_size;
    let (cnt_lumber, cnt_trees) = at_end_of_each[cycle_start+offset];
    println!("cycle start :{}, size: {}", cycle_start,cycle_size);

    let res : u64 = cnt_lumber as u64 * cnt_trees as u64;

    println!("result : {}x{}, {}", cnt_lumber, cnt_trees, res );
//

}

