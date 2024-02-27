#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#define MAX_LINE_LENGTH 1024

// Function to execute a command and read its output
bool execute_command(const char *command) {
    char buffer[MAX_LINE_LENGTH];
    bool success = true;

    printf("Executing command: %s\n", command);

    // Open a pipe to the command
    FILE *pipe = popen(command, "r");
    if (!pipe) {
        perror("Error executing command");
        return false;
    }

    // Read the output of the command line by line
    while (fgets(buffer, MAX_LINE_LENGTH, pipe) != NULL) {
        printf("%s", buffer);
    }

    // Check if the command execution was successful
    if (pclose(pipe) != 0) {
        printf("Command execution failed\n");
        success = false;
    }

    return success;
}

int main() {
    // Command to check the status of the service
    const char *status_cmd = "systemctl status btc-bot.service";

    // Command to show the system's handling of requests
    const char *journal_cmd = "journalctl -u btc-bot.service";

    printf("Checking the status of the BTC Bot service...\n");
    bool status_success = execute_command(status_cmd);

    printf("\n\nChecking the system's handling of requests for the BTC Bot service...\n");
    bool journal_success = execute_command(journal_cmd);

    // Check if both commands were successful
    if (status_success && journal_success) {
        printf("\nDone.\n");
        return 0;
    } else {
        printf("\nFailed to complete the operation.\n");
        return 1;
    }
}
