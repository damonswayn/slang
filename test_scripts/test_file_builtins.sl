let f = file_open("test.txt", "w+");
file_write(f, "Hello, world!");
file_seek(f, 0, "start");
print(file_read(f));
file_close(f);