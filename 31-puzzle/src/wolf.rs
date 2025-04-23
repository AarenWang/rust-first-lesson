use std::fmt::Debug;
use std::collections::HashSet;

// 定义物品类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Item {
    Wolf,
    Sheep,
    Cabbage,
}

// 定义河岸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Bank {
    Left,
    Right,
}

// 定义状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    wolf: Bank,
    sheep: Bank,
    cabbage: Bank,
    farmer: Bank,
}

impl State {
    fn is_valid(&self) -> bool {
        // 狼和羊不能单独在一边
        if self.wolf == self.sheep && self.farmer != self.wolf {
            return false;
        }
        // 羊和菜不能单独在一边
        if self.sheep == self.cabbage && self.farmer != self.sheep {
            return false;
        }
        true
    }

    fn new() -> Self {
        State {
            wolf: Bank::Left,
            sheep: Bank::Left,
            cabbage: Bank::Left,
            farmer: Bank::Left,
        }
    }

    // 尝试移动一个物品
    fn move_item(&self, item: Item) -> Option<Self> {
        // 确保物品和农夫在同一岸
        let item_bank = match item {
            Item::Wolf => self.wolf,
            Item::Sheep => self.sheep,
            Item::Cabbage => self.cabbage,
        };
        
        if item_bank != self.farmer {
            return None;
        }

        let mut new_state = *self;
        new_state.farmer = match self.farmer {
            Bank::Left => Bank::Right,
            Bank::Right => Bank::Left,
        };

        match item {
            Item::Wolf => new_state.wolf = new_state.farmer,
            Item::Sheep => new_state.sheep = new_state.farmer,
            Item::Cabbage => new_state.cabbage = new_state.farmer,
        }

        if new_state.is_valid() {
            Some(new_state)
        } else {
            None
        }
    }

    // 农夫单独移动
    fn move_farmer(&self) -> Option<Self> {
        let mut new_state = *self;
        new_state.farmer = match self.farmer {
            Bank::Left => Bank::Right,
            Bank::Right => Bank::Left,
        };

        if new_state.is_valid() {
            Some(new_state)
        } else {
            None
        }
    }

    fn is_goal(&self) -> bool {
        self.wolf == Bank::Right 
        && self.sheep == Bank::Right 
        && self.cabbage == Bank::Right 
        && self.farmer == Bank::Right
    }
}

// 解决问题的函数
fn solve() -> Option<Vec<State>> {
    let initial_state = State::new();
    let mut visited = HashSet::new();
    visited.insert(initial_state);
    
    let mut stack = vec![(initial_state, vec![initial_state])];
    
    while let Some((current_state, path)) = stack.pop() {
        if current_state.is_goal() {
            return Some(path);
        }
        
        // 尝试移动每个物品
        for &item in &[Item::Wolf, Item::Sheep, Item::Cabbage] {
            if let Some(new_state) = current_state.move_item(item) {
                if !visited.contains(&new_state) {
                    visited.insert(new_state);
                    let mut new_path = path.clone();
                    new_path.push(new_state);
                    stack.push((new_state, new_path));
                }
            }
        }
        
        // 尝试单独移动农夫
        if let Some(new_state) = current_state.move_farmer() {
            if !visited.contains(&new_state) {
                visited.insert(new_state);
                let mut new_path = path.clone();
                new_path.push(new_state);
                stack.push((new_state, new_path));
            }
        }
    }
    
    None
}

fn main() {
    println!("解决狼、羊、菜过河问题：");
    
    if let Some(solution) = solve() {
        println!("\n解法路径：");
        for (i, state) in solution.iter().enumerate() {
            println!("步骤 {}: {:?}", i, state);
        }
    } else {
        println!("无解！");
    }
}