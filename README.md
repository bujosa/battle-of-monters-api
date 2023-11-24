# Rust Coding Challenge - Battle of Monsters
FSL Version: V1.0.0

## Introduction
Welcome to our coding challenge! This challenge has been designed to test your coding skills and problem-solving abilities. This challenge is a great way to push yourself and learn new things. The challenge will test your technology stack skills and your ability to solve problems. Good luck, and have fun!

## Guidelines
You will create a screen recording video of yourself completing the challenge, then send us a link to the file via Google Drive. A few things to consider:

- We ask that you complete this challenge within the timeframe agreed on in our conversation.
- You can use screen recording software like Loom, QuickTime, or something similar, to create the video.
- As you complete the challenge, please explain what you are doing. Walk us through your thinking, explain your decisions, etc. Show us your UI work, if applicable.
- You cannot use tools such as Copilot, Tabnine, Captain Stack, GPT-Code-Clippy, chatGPT, or similar to simplify or generate code to support the challenge. Doing this will be grounds for automatic disqualification.
- You can use official documentation of the different tools you use for the challenge.
- You MUST NOT edit your video, stop it and continue later, copy contents from hidden screens, or do anything that could be considered cheating.
- The recording must be done in one take without pauses or editing. You MUST NOT stop or interrupt the recording at any point; if you do, you will be disqualified.
- You should start the recording after completing the project's environment setup.
- You should record your entire screen so we can validate your implementation correctly. Also, your computer clock should be visible in the entire video.
- Here is a short clip from a recent coding challenge as an example of what your recording should look like Example video. It is from a React challenge, but it is the same for any challenge.
- Please upload the video file to Google Drive and share an open link with us (we support .mp4, files smaller/with less than 4 GB).
- The recording should be of the coding challenge, from beginning to end, which is about 1 hour.

## Technologies
This project is built using the Rust ecosystem libraries; it is good you know the following items to have a good performance:
- Rust
- Actix
- Diesel
- Docker
- PostgreSQL

## The Coding Challenge
The app is a battle of monsters, where we have different monsters with different stats like attack and defense, for example, and we can let them fight each other.
We have implemented almost the entire CRUD for the Battle of Monsters app and have a battle endpoint to list all battles.

## Goals
- Implement missing functionalities: endpoints to list all monsters, start a battle, and delete a battle.
- Work on tests marked with TODO.
- Ensure the code style check script passes.

## Important Considerations
- Do NOT modify already implemented tests. If your code is implemented correctly, these tests should pass without modifications.
- You will face some issues in making the app run; this is part of the challenge, and we expect you to fix them.

## Battle Algorithm ⚔️
For calculating the battle algorithm, take into account the flow below:
- The monster with the highest speed makes the first attack, if both speeds are equal, the monster with the higher attack goes first.
- For calculating the damage, subtract the defense from the attack (attack - defense); the difference is the damage; if the attack is equal to or lower than the defense, the damage is 1.
- Subtract the damage from the HP (HP = HP - damage).
- Monsters will battle in turns until one wins; all turns should be calculated in the same request; for that reason, the battle endpoint should return winner data in just one call.
- Who wins the battle is the monster who subtracted the enemy’s HP to zero

## Project Setup
Numerical list of all the steps to follow to run the application
1. Clone repository on your terminal
   - GITHUB-TOKEN: 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
   - git clone https://{GITHUB-TOKEN}@github.com/fullstacklabs/assessment-cc-rust-sr-01.git --branch v1.0.0
   - Please refrain from using a GUI tool to clone the provided link. Also, ensure not to remove the token from the URL as doing so will prompt git to request a password, and you will not have it.
2. PostgreSQL database set up
   - docker-compose -f docker-postgres.yaml up -d
3. Database migrations
   - diesel migration run
​
## Project Usage
- Run the app
  - cargo run
- Run the tests
  - cargo test
​
## Acceptance Criteria
- All monster endpoints were implemented and working correctly.
- All battle endpoints were implemented and working correctly.
- Failing old tests should pass.
- All TODO tests were implemented successfully.
- Test code coverage should be at least 80%, and you must run it and show it to us during the recording.
- The code style check script must pass on completion of the challenge without any modifications to the config.
