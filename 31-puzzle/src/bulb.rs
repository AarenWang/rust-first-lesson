use std::fmt;

// 1. 定义灯泡状态
#[derive(Debug, Clone, PartialEq)]
enum BulbState {
    On,
    Off,
    Warm, // 表示关闭但曾经亮过（发热）
}

impl fmt::Display for BulbState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BulbState::On => write!(f, "On"),
            BulbState::Off => write!(f, "Off"),
            BulbState::Warm => write!(f, "Warm (previously On)"),
        }
    }
}

// 2. 定义灯泡行为（Trait）
trait BulbLike {
    fn change_state(&mut self);
    fn get_state(&self) -> BulbState;
}

// 3. 实现灯泡
#[derive(Debug)]
struct Bulb {
    state: BulbState,
}

impl BulbLike for Bulb {
    fn change_state(&mut self) {
        self.state = match self.state {
            BulbState::On => BulbState::Off,
            BulbState::Off => BulbState::On,
            BulbState::Warm => BulbState::On,
        };
    }

    fn get_state(&self) -> BulbState {
        self.state.clone()
    }
}

// 4. 定义开关（泛型 + Trait）
trait Switch<T: BulbLike> {
    fn toggle(&mut self, bulb: &mut T);
}

#[derive(Debug)]
struct LightSwitch {
    id: String,
}

impl<T: BulbLike> Switch<T> for LightSwitch {
    fn toggle(&mut self, bulb: &mut T) {
        bulb.change_state();
    }
}

// 5. 模拟实验
fn solve_bulb_puzzle() {
    // 初始化三个灯泡
    let mut bulb_a = Bulb { state: BulbState::Off };
    let mut bulb_b = Bulb { state: BulbState::Off };
    let mut bulb_c = Bulb { state: BulbState::Off };

    // 初始化三个开关
    let mut switch_a = LightSwitch { id: "SwitchA".to_string() };
    let mut switch_b = LightSwitch { id: "SwitchB".to_string() };
    let mut switch_c = LightSwitch { id: "SwitchC".to_string() };

    // Step 1: 打开 SwitchA 并保持一段时间（灯泡变热）
    switch_a.toggle(&mut bulb_a);
    bulb_a.state = BulbState::Warm; // 模拟灯泡发热

    // Step 2: 关闭 SwitchA，打开 SwitchB
    switch_a.toggle(&mut bulb_a);
    switch_b.toggle(&mut bulb_b);

    // Step 3: 检查灯泡状态
    println!("Bulb A: {}", bulb_a.get_state()); // Warm (SwitchA)
    println!("Bulb B: {}", bulb_b.get_state()); // On (SwitchB)
    println!("Bulb C: {}", bulb_c.get_state()); // Off (SwitchC)
}

// 6. 主函数
fn main() {
    println!("Solving the Three Bulbs and Switches Puzzle:");
    solve_bulb_puzzle();
}