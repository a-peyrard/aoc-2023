use std::collections::{HashSet, VecDeque};

pub fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

pub fn dump_graph(graph: &[Vec<u8>]) {
    for row in graph.iter() {
        for elem in row.iter() {
            print!("{}", *elem as char);
        }
        println!();
    }
}

pub fn find_islands(graph: &mut Vec<Vec<u8>>) -> Vec<u32> {
    // println!("\n\n=== Finding islands...");
    // dump_graph(graph);

    let mut islands = Vec::new();
    let height = graph.len();
    let width = graph[0].len();

    for j in 0..height {
        for i in 0..width {
            if graph[j][i] != b'X' {
                // println!("\n\n=== Try to find island starting at ({}, {})", i, j);
                // dump_graph(graph);

                if let Some(island_size) = find_island(graph, i, j) {
                    // println!("=> Found island of size {}", island_size);
                    islands.push(island_size);
                }
                // else {
                //     println!("=> Nothing found");
                // }
            }
        }
    }

    // println!("\n\n=== Islands found? {}", islands.len());
    // dump_graph(graph);

    islands
}

/*
procedure BFS(G, root) is
 2      let Q be a queue
 3      label root as explored
 4      Q.enqueue(root)
 5      while Q is not empty do
 6          v := Q.dequeue()
 7          if v is the goal then
 8              return v
 9          for all edges from v to w in G.adjacentEdges(v) do
10              if w is not labeled as explored then
11                  label w as explored
12                  w.parent := v
13                  Q.enqueue(w)
 */
pub fn find_island(graph: &mut Vec<Vec<u8>>, x: usize, y: usize) -> Option<u32> {
    let height = graph.len();
    let width = graph[0].len();

    let mut island_size = 0;
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    queue.push_back((x, y));

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        graph[y][x] = b'X';
        island_size += 1;

        // add neighbors to queue
        // north
        if y > 0 {
            if !seen.contains(&(x, y - 1)) && graph[y - 1][x] != b'X' {
                seen.insert((x, y - 1));
                queue.push_back((x, y - 1));
            }
        } else {
            return None;
        }
        // east
        if x < width - 1 {
            if !seen.contains(&(x + 1, y)) && graph[y][x + 1] != b'X' {
                seen.insert((x + 1, y));
                queue.push_back((x + 1, y));
            }
        } else {
            return None;
        }
        // south
        if y < height - 1 {
            if !seen.contains(&(x, y + 1)) && graph[y + 1][x] != b'X' {
                seen.insert((x, y + 1));
                queue.push_back((x, y + 1));
            }
        } else {
            return None;
        }
        // west
        if x > 0 {
            if !seen.contains(&(x - 1, y)) && graph[y][x - 1] != b'X' {
                seen.insert((x - 1, y));
                queue.push_back((x - 1, y));
            }
        } else {
            return None;
        }
    }

    // println!("=> Found island of size {} in ({}, {})", island_size, x, y);
    Some(island_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_find_islands_in_example1() {
        let mut graph = parse(
            "FXXXXXXXXXXXXXXXXXXX
LXXXXXXXXXXXXXXXXXXX
FXXXXXXXXXXXXXXXXXX7
XXXXXXXXXXXXXX7XXXX-
XXXXXXXXXX.||-XXXXJ7
|F|XXXXXXXXF7-XXL|7|
|FXXXXXXXXXXX|JXXXXX
7-XXXXXXXXXXXXXXXXXX
L.L7LXXXXXXXXXXXXXXX
L7JLJXXXXXXXXXXXXX.L",
        );

        let result = find_islands(&mut graph);
        assert_eq!(result.iter().sum::<u32>(), 10);
    }

    #[test]
    fn test_should_find_island_in_example1() {
        let mut graph = parse(
            "XXXXXXXXXXXXXXXXXXXX
XXXXXXXXXXXXXXXXXXXX
XXXXXXXXXXXXXXXXXXXX
XXXXXXXXXXXXXXXXXXXX
XXXXXXXXXX.||-XXXXJ7
|F|XXXXXXXXF7-XXL|7|
|FXXXXXXXXXXX|JXXXXX
7-XXXXXXXXXXXXXXXXXX
L.L7LXXXXXXXXXXXXXXX
L7JLJXXXXXXXXXXXXX.L",
        );

        let result = find_island(&mut graph, 10, 4);
        assert_eq!(result, Some(9));
    }
}
