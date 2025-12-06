#ifndef UTIL_H
#define UTIL_H

// returns the length of the string (terminated by \0 or \n)
int str_len(char* str);
// combine following characters into one -> "this is    a test" -> combine() -> "this is a test" and reallocs to new length
void combine(char** str, int* len, char c);
// removes whitespaces at the end and beginning
void trim(char** str, int* len);
// split string separated by delim into array
char** str_split(char* str, int len, char delim);
// delete an element from an array and left-shift the rest
void del(char** arr, int* len, int idx);
// counts lines in a file
int get_lines(char* filename);
// counts chars until delimiter
int chars_till_delim(char* str, int start, int len, char delim);
// counts occurences in string
int count_char(char* str, int len, char c);
// copy part of string
void str_cpy(char* src, char** dst, int start, int len);
// contains
int contains(char* str, int len, char c);

#endif