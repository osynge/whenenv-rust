* Example usage

Example useage with included data

    $ ./src/rust/target/debug/whenenv \
        --dir-sh src/data/ad404baf-4148-4a39-9746-a3a1adbc0683/shell \
        --dir-jobs src/data/ad404baf-4148-4a39-9746-a3a1adbc0683/job/ \
        --list-provides

This application is based upon sqlite 

To connect to database:

     $ sqlite3   -header      file.db
     SQLite version 3.16.2 2017-01-06 16:32:41
     Enter ".help" for usage hints.
     sqlite>


the way to get teh table list is


    sqlite> SELECT name FROM sqlite_master WHERE type = "table";
