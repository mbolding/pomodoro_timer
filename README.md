# Pomodoro Timer

A simple Pomodoro Timer application written in Rust. This application helps users manage their time effectively by implementing the Pomodoro Technique, which involves working for 25 minutes followed by a short break.

## Features

- Start a 25-minute work session
- Take a 5-minute short break
- Take a 15-minute long break
- View session logs
- Sound alerts and notifications upon session completion

## Getting Started

### Prerequisites

Make sure you have Rust installed on your machine. You can install Rust using [rustup](https://rustup.rs/).

### Building the Project

1. Clone the repository:

   ```
   git clone <repository-url>
   cd pomodoro_timer
   ```

2. Build the project using Cargo:

   ```
   cargo build
   ```

### Running the Application

To run the Pomodoro Timer, use the following command:

```
cargo run
```

Follow the on-screen instructions to start your work sessions and breaks.

## Logging

The application logs each session to a file named `pomodoro_log.txt`. You can view your session history by selecting the appropriate option in the menu.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.