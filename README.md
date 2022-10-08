# DroppingOut
Hack OHI/O 2022 Project

## Members
- Silas Springer
- Reilly Schultz
- Justin Garey

## The project
We are making a discord bot to transcribe and translate the discord voice chat. We are making use of
a rust backend hosted on an AWS EC2 instance.

### EC2 Instance Specs
- t2.micro with 1vCPU and 1 GiB of memory
- 30 GiB EBS volume
- Amazon Linux for the OS

### System Requirements
- Need python/pip, rust and whisper installed
- (Whisper)[https://github.com/openai/whisper]
  - Use the pip command from their README 