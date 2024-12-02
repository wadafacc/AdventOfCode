IDENTIFICATION DIVISION.
PROGRAM-ID. DayTwo.
ENVIRONMENT DIVISION.
INPUT-OUTPUT SECTION.
FILE-CONTROL.
    SELECT InputFile ASSIGN TO 'inputs.txt'
        ORGANIZATION IS LINE SEQUENTIAL.

DATA DIVISION.
FILE SECTION.
FD InputFile.
01 Ln PIC X(100).

WORKING-STORAGE SECTION.
01 EOF          PIC X VALUE 'N'.
01 Result PIC 9(1).
01 Counter PIC 9(5) VALUE 0.

PROCEDURE DIVISION.
    OPEN INPUT InputFile
    PERFORM UNTIL EOF = 'Y'
        READ InputFile INTO Ln
            AT END
                MOVE 'Y' TO EOF
            NOT AT END
              *>  DISPLAY Ln
               CALL 'EvalLn' USING Ln, Result
               COMPUTE Counter = Counter + Result          
        END-READ
    END-PERFORM
    CLOSE InputFile


    DISPLAY "COUNTER VALUE"
    DISPLAY Counter
STOP RUN.
