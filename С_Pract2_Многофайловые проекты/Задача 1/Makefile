CC = gcc
CFLAGS = -Wall -O2
SRC = main.c student.c
OBJ = $(SRC:.c=.o)
TARGET = student_manager.exe

all: $(TARGET)

$(TARGET): $(OBJ)
	$(CC) -o $@ $^ $(CFLAGS)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJ) $(TARGET)

run: $(TARGET)
	./$(TARGET)

.PHONY: all clean run