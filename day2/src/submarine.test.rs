#[cfg(test)]
mod test_submarine {
    use super::super::*;  

    #[test]
    fn parse_command_down() {
      assert_eq!(Command::from_str("down 10").unwrap(), Command::Down(10));
    }

    #[test]
    fn parse_command_up() {
      assert_eq!(Command::from_str("up 10").unwrap(), Command::Up(10));
    }

    #[test]
    fn parse_command_forward() {
      assert_eq!(Command::from_str("forward 10").unwrap(), Command::Forward(10));
    }

    #[test]
    fn position_product() {
      assert_eq!(Submarine { horizontal: 0, depth: 0, aim: 0 }.position_product(), 0);
      assert_eq!(Submarine { horizontal: 10, depth: 2, aim: 0 }.position_product(), 20);
      assert_eq!(Submarine { horizontal: 10, depth: 2, aim: 5 }.position_product(), 20);
    }

    #[test]
    fn perform_down_command() {
      let submarine = Submarine::new().perform_command(&Command::Down(10));
      assert_eq!(submarine.depth, 10);
      assert_eq!(submarine.horizontal, 0);
    }

    #[test]
    fn perform_down_command_with_aim() {
      let submarine = Submarine::new().perform_command_with_aim(&Command::Down(10));
      assert_eq!(submarine.depth, 0);
      assert_eq!(submarine.horizontal, 0);
      assert_eq!(submarine.aim, 10);
    }

    #[test]
    fn perform_up_command() {
      let submarine = Submarine::new().perform_command(&Command::Up(10));
      assert_eq!(submarine.depth, -10);
      assert_eq!(submarine.horizontal, 0);
    }

    #[test]
    fn perform_up_command_with_aim() {
      let submarine = Submarine::new().perform_command_with_aim(&Command::Up(10));
      assert_eq!(submarine.depth, 0);
      assert_eq!(submarine.horizontal, 0);
      assert_eq!(submarine.aim, -10);
    }

    #[test]
    fn perform_forward_command() {
      let submarine = Submarine::new().perform_command(&Command::Forward(10));
      assert_eq!(submarine.depth, 0);
      assert_eq!(submarine.horizontal, 10);
    }

    #[test]
    fn perform_forward_command_with_aim() {
      let submarine = Submarine::new().perform_command_with_aim(&Command::Forward(10));
      assert_eq!(submarine.depth, 0);
      assert_eq!(submarine.horizontal, 10);
      let submarine = Submarine { horizontal: 0, depth: 0, aim: 10 }.perform_command_with_aim(&Command::Forward(10));
      assert_eq!(submarine.depth, 100);
      assert_eq!(submarine.horizontal, 10);
    }
}
