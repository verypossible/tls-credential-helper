# Purpose

## What

TCH is a CLI that creates key pairs and certificates for use with TLS.

## Why

The OpenSSL CLI is modular and encompassing which makes it unapproachable and error-prone for the
non-veteran user.

## Values

### Minimal API surface area

The more commands and options there are for a tool, the more friction there is for a user to
understand if they want to use it and how. If you make it hard for the user to do the right thing,
you reap what you sow.

### Informative and interactive UX

CLIs can make complex tasks easy, but they should keep the user appropriately informed and prompt
for confirmation when taking notable action. This aids understanding and reduces mistakes.

### Opinionated workflows

The value of this tool is providing safe and simple commands that more simply achieve what would
otherwise be a multi-step process.

### Small and portable binary

TCH supports Linux, MacOS, and Windows on consumer, server, and constrained embedded hardware.
