<!-- @format -->

# Rocket 💫 TypeScript

![License](https://img.shields.io/github/license/Kindness-Works/rocket-ts)

**Rocket-TS** is a tool designed to rapidly generate TypeScript interfaces from [Rocket](https://rocket.rs) backend 🦀. It simplifies the integration of your Rust backend with a TypeScript frontend by automatically generating TypeScript interfaces for Rocket request handlers.

Inspired by [TypeShare](https://github.com/1Password/typeshare), it aims to enhance the developer experience for frontend engineers. 🤗

## Features

-   Generates TypeScript interfaces for Rocket request handlers.
-   Supports exclusion of parameters from the generated interface, useful for omitting [Request Guards](https://rocket.rs/guide/v0.4/requests/#request-guards).
-   Offers a flexible command-line interface.
-   Ensures fast and efficient generation.

## Installation

1. Ensure Rust is installed on your system. If not, download and install it from the official Rust website: [Rust Installation Guide](https://www.rust-lang.org/tools/install).

2. Clone this repository:

```bash
git clone https://github.com/Kindness-Works/rocket-ts.git
```

3. Navigate to the project directory:

```bash
cd rocket-ts
```

4. Build the project:

```bash
cargo build --release
```

The binary will be available in the `target/release` directory.

## Usage

To generate TypeScript interfaces, utilize the `generate` subcommand:

```bash
rocket-ts generate -i <INPUT> -o <OUTPUT> [-e <EXCLUDE_FILE>]
```

-   `-i`, `--input`: Specifies the input directory or file to parse for interface generation.
-   `-o`, `--output`: Indicates the output file for the generated interface.
-   `-e`, `--exclude` (optional): Specifies a file containing parameters to exclude from the interface, such as Request Guards.

## Example

Suppose you have a Rocket project structured as follows:

```
my-rocket-project/
├── src/
│   ├── main.rs
│   ├── routes/
│   │   ├── users.rs
│   │   └── posts.rs
│   └── guards/
│       └── auth.rs
└── exclude.txt
```

To generate TypeScript interfaces for the `users` and `posts` modules, excluding parameters specified in `exclude.txt`, run:

```bash
rocket-ts generate -i src/routes -o interfaces.ts -e exclude.txt
```

This command generates a `interfaces.ts` file containing TypeScript interfaces for the request handlers in `users.rs` and `posts.rs`, excluding parameters specified in `exclude.txt` (e.g., the `auth` guard).

---

Example interface.ts generated from the [`example-handlers`](https://github.com/Kindness-Works/rocket-ts/tree/main/example-handlers) folder in the repository:

Now, in the Rocket file [`thread.rs`](https://github.com/Kindness-Works/rocket-ts/blob/main/example-handlers/thread.rs), there is `AgentService`, a [Request Guard](https://rocket.rs/guide/v0.4/requests/#request-guards) that we don't want to include in the generated types, as Rocket internally takes care of it. We can create an `exclude.txt` file to exclude it with the following content:

```txt
AgentService
```

Then run:

```shell
rocket-ts generate -i example-handlers -o example.ts -e exclude.txt
```

This will generate the following TypeScript interface:

```typescript
/*
 * Generated by rocket-ts 0.1.0 🚀 🌎
 */

export interface k7 {
	// thread.rs
	// Route: "/thread/<kid_or_cerb_mask>"
	get_thread: () => Thread;
	// Route: "/debug/thread/<kid>"
	get_thread_debug: () => ThreadDebug;
	// Route: "/thread/<thread_id>/comments"
	get_thread_comments: (thread_id: i32) => Comment[];
	// Route: "/thread/<thread_id>/insights"
	get_thread_insights: (thread_id: i32) => MessageInsights;
	// Route: "/thread/escalate"
	escalate_thread: (ThreadEscalation) => any;
}
```

## Contributing

Contributions are welcomed! Feel free to open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).
