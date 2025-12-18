/* OASM Native C Library - FFI Primitives */
#include <stdio.h>
#include <stdint.h>
#include <string.h>

/* Basic arithmetic (example FFI function) */
int oasm_add(int a, int b) {
    return a + b;
}

/* String length (safe C string operation) */
int oasm_strlen(const char* str) {
    if (!str) return -1;
    return (int)strlen(str);
}

/* Memory copy with bounds checking */
int oasm_memcpy_safe(void* dest, const void* src, size_t n, size_t dest_size) {
    if (!dest || !src) return -1;
    if (n > dest_size) return -2;  // Buffer overflow protection
    memcpy(dest, src, n);
    return 0;
}

/* Task state encoding for supervisor (executive function support) */
typedef enum {
    TASK_PENDING = 0,
    TASK_IN_PROGRESS = 1,
    TASK_COMPLETED = 2,
    TASK_FAILED = 3
} TaskState;

typedef struct {
    uint32_t task_id;
    TaskState state;
    uint64_t timestamp;
    char description[256];
} Task;

/* Get task state as string (for UI display) */
const char* oasm_task_state_str(TaskState state) {
    switch (state) {
        case TASK_PENDING: return "Pending";
        case TASK_IN_PROGRESS: return "In Progress";
        case TASK_COMPLETED: return "Completed";
        case TASK_FAILED: return "Failed";
        default: return "Unknown";
    }
}

/* Capability check (stub - real impl would check Windows security tokens) */
int oasm_check_capability(const char* cap_name) {
    // Placeholder: In production, check actual Windows capability/token
    printf("[NATIVE] Checking capability: %s\n", cap_name);
    return 1;  // Allow for now
}
