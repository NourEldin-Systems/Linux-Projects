#include "../include/colors.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

const short MAX_NAME_LEN = 50;

enum ParserState { ExpectsCommand, ExpectsArgument, Executes };

typedef struct {
  char *items[50];
  short last_index;
} Queue;

int main() {
  Queue queue = {.last_index = -1};

  // command to be parsed
  char command[50];
  // new patient's name to add to the queue
  char new_name[100];

  while (1) {
    if (fgets(command, 100, stdin)) {
      char *token = strtok(command, " ");

      while (token != NULL) {
        // parse(token);
        token = strtok(NULL, " ");
      }
    }

    // Read the new name and add it to the queue items
    printf("Add a new patient (#%d)", queue.last_index + 1);
    if (fgets(new_name, 50, stdin)) {
      new_name[strcspn(new_name, "\n")] = '\0';

      if (queue.last_index + 1 < sizeof(queue.items)) {
        queue.items[queue.last_index + 1] = strdup(new_name);
        queue.last_index++;
      } else {
        printf("Can't add the item, the queue is already full\n");
      }
    }
    printf("%s was successfully added to the queue, queue number is %d#\n",
           new_name, queue.last_index + 1);

    // Printing all the names in the queue at after adding a new name
    printf("All the names are :\n");
    for (short i = 0; i < (sizeof(queue.items) / sizeof(queue.items[0])); i++) {
      if (queue.items[i] != NULL) {
        printf("%d# - %s\n", i + 1, queue.items[i]);
      }
    }
  }

  // Freeing up memory before the program ends
  for (int i = 0; i < queue.last_index; i++) {
    free(queue.items[i]);
  }

  return 0;
}

int parse(char *token) {}
