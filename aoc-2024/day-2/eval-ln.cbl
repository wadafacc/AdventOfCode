IDENTIFICATION DIVISION.
       PROGRAM-ID. EvalLn.

       DATA DIVISION.
       LOCAL-STORAGE SECTION.
           01 Idx           PIC 9(5)   VALUE 0.
           01 RIdx          PIC 9(5).
           01 NextIdx       PIC 9(5)   VALUE 0.
           01 PrevSign      PIC S9(1)  VALUE 0.
           01 CurrentSign   PIC S9(1)  VALUE 0.
           01 Counter       PIC 9(1)   VALUE 0.
           01 TempDiff      PIC 9(5)   VALUE 0.
           01 Diff          PIC S9(5)  VALUE 0.

           01 Items.
               05 Item      OCCURS 10 TIMES
                            PIC 9(3) VALUE 0.

           01 Len           PIC 9(5)   VALUE 0.

       LINKAGE SECTION.
           01 Ln            PIC X(100).
           01 Result        PIC 9(1). *> 0 = Unsafe, 1 = Safe

       PROCEDURE DIVISION USING Ln, Result.

           PERFORM MainSection.

       MainSection SECTION.
           *> Unstring Input
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

           *> Initialize CurrentSign
           COMPUTE CurrentSign = 
                    FUNCTION MIN(1, FUNCTION MAX(Item(1) - Item(2), -1))

           *> Main Loop
           PERFORM VARYING Idx FROM 1 BY 1 UNTIL Idx >= 10
               PERFORM Handling
           END-PERFORM

           *> Reset Items
           INITIALIZE Items
       EXIT.

      *>******************************************************************
       Handling SECTION.
      *>******************************************************************
           *> Calculate Next Index
           COMPUTE NextIdx = Idx + 1
           IF NextIdx > 8 THEN
               CONTINUE
           END-IF

           *> Sign Calculation
           COMPUTE PREVSIGN = 
            FUNCTION MIN(1, FUNCTION MAX(Item(Idx) - Item(NextIdx), -1))

           *> Check for Zero
           IF Item(Idx) = 0 OR Item(NextIdx) = 0 THEN
               COMPUTE Result = 1
               DISPLAY "RETURNING 1"
               EXIT PROGRAM
           END-IF

           *> Calculate Difference
           COMPUTE TempDiff = FUNCTION ABS(Item(Idx) - Item(NextIdx))

           *> Logging (commented out for production)
           DISPLAY "ITEM: " Item(Idx) 
                   " | ITEM +1: " Item(NextIdx) 
                   " | DIFF: " TempDiff 
                   " | SIGN: " CurrentSign 
                   " | PREVSIGN: " PrevSign

           *> Validate Difference and Sign
           IF TempDiff > 3 OR TempDiff < 1 OR PrevSign NOT EQUAL TO CurrentSign THEN
               COMPUTE Counter = Counter + 1

               *> Handle Safe Transition
               IF Counter = 1 THEN

                   PERFORM VARYING RIdx FROM Idx BY 1 UNTIL RIdx >= 10
                       MOVE Item(RIdx + 1) TO Item(RIdx)
                   END-PERFORM
                   MOVE 0 TO Item(10)

                   MOVE 0 to Idx
                   DISPLAY "COUNTER INCREASED; RESTARTING: " Items
                   *> Initialize CurrentSign
                   COMPUTE CurrentSign = 
                       FUNCTION MIN(1, FUNCTION MAX(Item(1) - Item(2), -1))
               END-IF
           END-IF

           *> Handle Unsafe Condition
           IF Counter > 1 THEN
               COMPUTE Result = 0
               DISPLAY "EXITING"
               EXIT PROGRAM
           END-IF
       EXIT.

      *>******************************************************************
       END PROGRAM EvalLn.
