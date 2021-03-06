<!-- PROJECT LOGO -->
<br />
<p align="center">
  <a href="https://github.com/wasabiegg/hanime">
    <img src="images/logo.jpg" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">Hanime Downloader</h3>

  <p align="center">
    An simple hanime command download tool written in rust!
    <br />
    <a href="https://github.com/wasabiegg/hanime/issues">Report Bug</a>
    ·
    <a href="https://github.com/wasabiegg/hanime/issues">Request Feature</a>
  </p>
</p>



<!-- TABLE OF CONTENTS -->
<details open="open">
  <summary>Table of Contents</summary>
  <ol>
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
  </ol>
</details>



<!-- GETTING STARTED -->
## Getting Started


### Prerequisites

* ffmpeg
* rust

### Installation

<!-- 1. Get a free API Key at [https://example.com](https://example.com) -->
1. Clone the repo
   ```sh
   git clone https://github.com/wasabiegg/hanime
   ```
2. Build
   ```sh
   cd hanime
   cargo build --release
   ```


<!-- USAGE EXAMPLES -->
## Usage

### specify url, save path and tmp path
```sh
  hanime get --url ********** --path ~/Downloads --tmp ~/Downloads
```

```sh
  hanime get --url ********** --p ~/Downloads --t ~/Downloads
```


### specify multiple url, default save path and tmp path is in executable dir
```sh
  hanime get --url ***********  ************ ************
```



<!-- ROADMAP -->
## Roadmap

See the [open issues](https://github.com/wasabiegg/hanime/issues) for a list of proposed features (and known issues).



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.



<!-- CONTACT -->
## Contact

<!-- Your Name - [@your_twitter](https://twitter.com/your_username) - email@example.com -->

Project Link: [https://github.com/wasabiegg/hanime](https://github.com/wasabiegg/hanime)