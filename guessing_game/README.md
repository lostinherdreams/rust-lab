# Rust Guessing Game

This is a simple number guessing game written in Rust. The program generates a random number between 1 and 100, and the user needs to guess that number within a limited number of attempts.

## Gameplay

The Rust Guessing Game is an interactive program where:

- The program generates a random number between 1 and 100.
- You will be prompted to guess the number.
- Input your guess and press Enter.
- The program will provide feedback if your guess is too small, too big, or correct.
- Keep guessing until you correctly guess the secret number!

## Code Explanation

The code is structured into different sections:

- **Welcome Message:** Displays a greeting message to the user.
- **Random Number Generation:** Uses the rand crate to generate a random number between 1 and 100.
- **Guessing Loop:** A loop that prompts the user to input their guess, provides feedback, and continues until the correct number is guessed.
- **Error Handling:** Handles non-numeric inputs gracefully by informing the user and allowing them to continue guessing.
- **Display of Secret Number:** Once the correct number is guessed, the secret number is revealed.

## Contributing

Contributions are welcome! Feel free to open issues, propose new features, or create pull requests.

If you find bugs or have suggestions to improve the game, please open an issue.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- This game was built following the instructions from [The Rust Programming Language](https://doc.rust-lang.org/book/) book.
- Thanks to the Rust community for providing valuable resources and support.

### Prerequisites

To run this program, ensure you have Rust installed. If Rust is not installed, you can get it from [Rust's official website](https://www.rust-lang.org/tools/install).

### Installation

Clone this repository to your local machine using Git:

```bash
git clone https://github.com/lostinherdreams/rust-guessing-game.git
```
