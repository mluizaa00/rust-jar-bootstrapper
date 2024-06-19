## Introduction

This project provides a modular and efficient way to run JAR applications within Docker containers.
It leverages Rust's powerful capabilities to create a bootstrapper that seamlessly fetches JAR and JSON configuration files from Amazon S3 storage. 
By following the instructions outlined in the JSON file, the bootstrapper executes the necessary commands to launch the JAR application within the container.

## Key Features

**Modularization**: Encourages code reusability and maintainability by separating JAR execution logic from configuration details.
**S3 Integration**: Provides a secure and scalable mechanism to store and retrieve JAR and configuration files.
**Customization**: The JSON configuration file allows you to tailor the execution environment for your specific JAR application needs.
