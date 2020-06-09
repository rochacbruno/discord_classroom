# Disclass

Discord bot to work as a Learning Management System - LMS

## Features

### Version 1

- [] Student goes to class channel and type `$join course-name`
       - bot answers with `user.name you joined class foo + help message`

- Quizz (checkbox)

  - [] Student sends `$quizz` and the bot checks the next unanswered quizz available
    for the user to solve. 
      ```
      Hi user.name,

      This quizz has 20 questions, once started you have 30 minutes to solve it.
      You have 3 chances.
      ```
  - [] Questions are sent one by one and user must add reaction on correct answer [1,2,3,4,5]
  - [] Bot listens to reactions and saves it on user database
  - [] Once finished last question bot calculates student grade
  - [] Bot checks the timeout


- Notifications
- Certificate


### Version 2

- Register new class
    - When a new role prefixed with `class-` is created
    The bot is going to create a new `classroom` based
    on templates. `class-template.yaml`

- Register new students
    - every time a member is included in `Student` and `class-` role they are going to be registered.
    - Bot is going to send a DM so the member should agree with some terms.
