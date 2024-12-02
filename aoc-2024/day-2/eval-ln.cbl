IDENTIFICATION DIVISION.
PROGRAM-ID. EvalLn.
DATA DIVISION.
LOCAL-STORAGE SECTION.
01 Idx PIC 9(5) VALUE 0.
01 NextIdx PIC 9(5) VALUE 0.
01 PrevSign PIC S9(1) VALUE 0.
01 CurrentSign PIC S9(1) VALUE 0.

01 TempDiff PIC 9(5) VALUE 0.
01 Diff PIC S9(5) VALUE 0.

01 Items.
   05 Item OCCURS 8 TIMES
       PIC 9(4) VALUE 0.
01 Len PIC 9(5) VALUE 0.
LINKAGE SECTION.
01 Ln PIC X(100).
01 Result PIC 9(1). *> 0 = Unsafe, 1 = Safe

PROCEDURE DIVISION USING Ln, Result.
           UNSTRING Ln DELIMITED BY ALL SPACES
               INTO Item(1)
                    Item(2)
                    Item(3)
                    Item(4)
                    Item(5)                             
                    Item(6)                             
                    Item(7)                             
                    Item(8)                             
           END-UNSTRING

       COMPUTE CurrentSign = FUNCTION MIN(FUNCTION MAX(Item(1) - Item(2) -1) 1) * -1
       PERFORM VARYING Idx FROM 1 BY 1 UNTIL Idx > 8
           COMPUTE NextIdx = Idx + 1


           *> Sign Calc
           COMPUTE PrevSign = FUNCTION MIN(FUNCTION MAX(Item(Idx) - Item(NextIdx) -1) 1) * -1

           IF Item(Idx) = 0 OR Item(NextIdx) = 0
               COMPUTE Result = 1
                *> DISPLAY "RETURNING 1"
               EXIT PROGRAM
           END-IF

           *> Calc Diff
           COMPUTE TempDiff = function ABS(Item(Idx) - Item(NextIdx))

           *> Fancy Logging :)
           DISPLAY "ITEM: " Item(Idx) " | ITEM +1: " Item(NextIdx) " | DIFF: " TempDiff " | SIGN: " CurrentSign " | PREVSIGN: " PrevSign

           IF TempDiff > 3 OR TempDiff < 1 OR PrevSign NOT EQUAL TO CurrentSign
               COMPUTE Result = 0
               DISPLAY "EXITING"
               EXIT PROGRAM
           END-IF
       END-PERFORM
       INITIALIZE Items
EXIT PROGRAM.
