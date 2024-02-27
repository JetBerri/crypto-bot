#!/bin/bash

# Service name
SERVICE_NAME="bot"

# Path to the Rust binary
BIN_PATH="/path/to/your/binary/bot"

# Path to the scripts directory
SCRIPT_DIR="/path/to/your/scripts/directory"

# Path to the run script
RUN_SCRIPT="$SCRIPT_DIR/run_bot.sh"

# Path to the systemd service file
SERVICE_FILE="/etc/systemd/system/$SERVICE_NAME.service"

# Function to handle errors and exit
handle_error() {
    local message="$1"
    echo "Error: $message"
    exit 1
}

# Check if the Rust binary exists
if [ ! -f "$BIN_PATH" ]; then
    handle_error "The Rust binary '$BIN_PATH' does not exist."
fi

# Create the run script
cat << EOF > "$RUN_SCRIPT"
#!/bin/bash

# Set the working directory
cd "\$(dirname "\$BIN_PATH")" || exit

# Execute the Rust binary
exec "\$BIN_PATH"
EOF

# Give execute permission to the run script
chmod +x "$RUN_SCRIPT" || handle_error "Failed to set execute permission on the run script."

# Move the systemd service file
mv "$SERVICE_FILE" "$SCRIPT_DIR" || handle_error "Failed to move the systemd service file."

# Reload systemd
systemctl daemon-reload || handle_error "Failed to reload systemd."

# Start the service
systemctl start "$SERVICE_NAME" || handle_error "Failed to start the '$SERVICE_NAME' service."

# Enable the service to start on boot
systemctl enable "$SERVICE_NAME" || handle_error "Failed to enable the '$SERVICE_NAME' service to start on boot."

# Check the status of the service
systemctl status "$SERVICE_NAME" || handle_error "Failed to get the status of the '$SERVICE_NAME' service."
