#!/bin/bash

# Function to display system information
display_system_info() {
    # Processor Information
    CPU_TYPE=$(lscpu | grep "^Architecture:" | awk '{print $2}')  # Assuming a static value
    CPU_CORES=$(lscpu | grep "^CPU(s):" | awk '{print $2}')
    CPU_CLOCK=$(lscpu | grep "Model name" | awk -F '@' '{print $2}' | xargs)
    cpu_usage=$(mpstat | grep -i "all" | awk '{print $3}')
    CPU_USAGE="$cpu_usage%"
    TEMP=$(cat /sys/class/thermal/thermal_zone0/temp)
    # CPU_TEMPERATURE=$(awk "BEGIN {printf \"%.1f°C\n\", $((TEMP/1000}))")
    CPU_TEMPERATURE=$(awk -v temp="$TEMP" 'BEGIN {printf "%.1f°C\n", temp/1000}')

    meminfo=$(cat /host_proc/meminfo)
    mem_total_kb=$(echo "$meminfo" | grep 'MemTotal' | awk '{print $2}')
    mem_free_kb=$(echo "$meminfo" | grep 'MemFree' | awk '{print $2}')

    # Convert from KB to GB
    mem_total_gb=$(echo "scale=2; $mem_total_kb/1024/1024" | bc)
    mem_free_gb=$(echo "scale=2; $mem_free_kb/1024/1024" | bc)
    # Memory Information
    printf -v formatted_mem_total_gb "%.2f" "$mem_total_gb"
    printf -v formatted_mem_free_gb "%.2f" "$mem_free_gb"
    RAM_INSTALLED="${formatted_mem_total_gb} GB"
    RAM_FREE="${formatted_mem_free_gb} GB"
    DISK_INSTALLED=$(df -H --output=size / | tail -n 1 | awk '{print $1}')
    DISK_FREE=$(df -H --output=avail / | tail -n 1 | awk '{print $1}')

    # Board Temperature - Assuming a static value
    BOARD_TEMPERATURE="42°C"  

    # Output as JSON
    echo -e "{
        \"HealthStatus\": \"NORMAL\",
        \"Hardware\": {
            \"Processor\": {
                \"CpuType\": \"$CPU_TYPE\",
                \"CpuCores\": \"$CPU_CORES\",
                \"CpuClock\": \"$CPU_CLOCK\",
                \"CpuUsage\": \"$CPU_USAGE\",
                \"CpuTemperature\": \"$CPU_TEMPERATURE\"
            },
            \"Memory\": {
                \"RAMInstalled\": \"$RAM_INSTALLED\",
                \"RAMFree\": \"$RAM_FREE\",
                \"DiskInstalled\": \"$DISK_INSTALLED\",
                \"DiskFree\": \"$DISK_FREE\"
            },
            \"BoardTemperature\": \"$BOARD_TEMPERATURE\"
        },
        \"LastUpdate\": \"$(date -u +'%Y-%m-%dT%H:%M:%S.%N%:z')\"
    }"
}
# Call the function to display the information
display_system_info