
# YAPPPWN

A Rust rewrite of the PPPwn exploit - PlayStation 4 PPPoE RCE by TheOfficialFloW [here](https://github.com/TheOfficialFloW/PPPwn)  

This project was created for fun and to learn Rust, so feel free to contribute!

Should work from 11.00 to 7.0.
I have tested this on:
- [X] 11.00 

## Run Locally

Build the project:

```
cargo build -r
```

and then run it

```
sudo ./target/release/yapppwn -h                                                                                                                                                      

[+] YAPPPWN [+]
YAPPPWN, Yet Another PPPwn (in Rust)

Usage: yapppwn --interface <INTERFACE> --fw <FW> --stage-1 <STAGE_1> --stage-2 <STAGE_2>

Options:
  -i, --interface <INTERFACE>  Interface where the ps4 is connected to
      --fw <FW>                Firmware version from 1100 (11.00) to 900 (9.00)
      --stage-1 <STAGE_1>      Stage 1 Payload Path
      --stage-2 <STAGE_2>      Stage 2 Payload Path
  -h, --help                   Print help
  -V, --version                Print version
```

## Known Bugs
- It is not endian agnostic atm.

## FAQ
### Why?
I wanted to learn Rust and was curious about the inner workings of the exploit by theFlow.

### Why did you use X instead of Y?  
I'm still learning. If Y is better than X, please feel free to open a pull request and explain!

## Authors

- [@fedebuonco](https://www.github.com/fedebuonco)

## Acks
Thanks to TheOfficialFloW for this amazing exploit.  
Thanks to [LowLevelLearning](https://www.youtube.com/lowlevellearning), whose video sparked my curiosity about this exploit.  
Thanks to Claude and ChatGPT, both very helpful, especially in setting up some unit tests.  
