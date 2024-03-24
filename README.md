<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<p align="center">
  <a href="https://github.com/CyberT3C/siqi">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">Siqi</h3>

</p>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>


<!-- ABOUT THE PROJECT -->
## About The Project
Creating a super simple task cli tool.

Project Management Tools are nice and super helpfull but they take alot of effort to setup and customize. Alot of the time the best way to star a project is by "just do it!". For that is like init git and create a readme.md file. I always end up bulding a todo.txt or todo.md or write the first todos as comments straight into my code file. To standardize this process i am creating this simple tool. If it will be handy, I will end up support integration to other tools with UI.

Also nice to have everything it git. Alot of the tools are using a database or a special file format. Thats why i am just using a simple .yaml notation

My longterm goal is to build a simple cli tool which will store tasks in a `default.task` file in the current directory.
Publish / Update tasks from default file into `README.md`.

PS: I want to learn rust and this is my project to do so.

### Built With

This section should list any major frameworks that you built your project using. Leave any add-ons/plugins for the acknowledgements section. Here are a few examples.
* [Neovim]()
* [rust-analyzer]()
* [rust]()
* [Nix]()


<!-- GETTING STARTED -->
## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Prerequisites

* make sure you have a cli installed e.g. bash, sh, zsh, powershell

### Installation

9. Clone the repo
   ```sh
   git clone https://github.com/CyberT3C/siqi
   ```
3. Install Rust
   ```sh
   ask your favorit AI 
   ```
4. Build
   ```sh
    cargo build
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->
## Usage

```bash 
Usage: command [options] <arguments>

Options
    list
    add <task-name>
    done <index>

# e.g.
siqi add "this is a new task"
siqi done 1
siqi list
```

Docs - missing link

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ROADMAP -->
## Roadmap
 
- [X] Initial Readme
- [X] Create local POC
- [ ] Create Alpha Version
    - [X] Build a task list 
    - [X] Save to file
    - [X] Read from file
    - [X] Add task
    - [X] Delete task
    - [X] View all tasks
    - [X] Mark task as done
    - [ ] Uncheck done task
    - [X] Create cli integration
        - [X] Add MoveUp Feature
        - [X] Add MoveDown Feature
- [ ] Code Review
- [ ] Refactor Codebase
- [ ] Add Changelog
- [ ] Build with nix
- [ ] Release v0.1
- [ ] Support different OSs
    - [ ] Linux
    - [ ] Windows
    - [ ] Mac

My longterm goal is to build a simple cli tool which will store tasks in a `default.task` file in the current directory.
Publish / Update tasks from `default.task` file into `README.md` - I am talking about "root" tasks.

Maybe: Integrate this tool with other task / project management tools, like Github, Gitlab or Jira.

See the [open issues](https://github.com/CyberT3C/siqi/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTACT -->
## Contact

CyberT3C - missing link

Project Link: [https://github.com/CyberT3C/siqi](https://github.com/CyberT3C/siqi)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

Resources which are helpful and would like to give credit to!

* [Choose an Open Source License](https://choosealicense.com)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/CyberT3C/siqi.svg?style=for-the-badge
[contributors-url]: https://github.com/CyberT3C/siqi/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/CyberT3C/siqi.svg?style=for-the-badge
[forks-url]: https://github.com/CyberT3C/siqi/network/members
[stars-shield]: https://img.shields.io/github/stars/siqi.svg?style=for-the-badge
[stars-url]: https://github.com/CyberT3C/siqi/stargazers
[issues-shield]: https://img.shields.io/github/issues/CyberT3C/siqi.svg?style=for-the-badge
[issues-url]: https://github.com/CyberT3C/siqi/issues
[license-shield]: https://img.shields.io/github/license/CyberT3C/siqi.svg?style=for-the-badge
[license-url]: https://github.com/CyberT3C/siqi/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/CyberT3C
[product-screenshot]: images/screenshot.png
