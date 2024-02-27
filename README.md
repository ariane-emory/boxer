# A thing that almost parses ASCII-formatted IEC-61131-3 Function Block Diagrams

It doesn't have vertical connections, the ability for connections to cross over each other, or airwire I/O source/sinks yet, but they're coming soon.

```
         +---------+                         +-----------+
 +---+   |  THING  |-------------------------|CS         |
 | A |---|         |          +--------+     |           |
 |   |   |         |          |        |     |  SPIDAC   |
 +---+   +---------+          | ALMOST |     |           |
                 +---------+  |        |-----|SCK    LOUT|
+------------+   |   CAN   |--|        |     |           |
|    THAT    |---|         |  +--------+     |       ROUT|
|            |   +---------+                 |INT1       |
+------------+   +-----------+               |           |
                 | IEC6113-3 |---------------|MOSI       |
    +-------+    |           |   +-------+   |           |
    | PARSE |----|           |   | FBDs  |---|MISO       |
    |       |    |           |---|       |   |           |
    +-------+    +-----------+   +-------+   +-----------+
                                               

#1 HorizConn {
    begin: Column #20, Line #1,
    length: 24,
    state: Finished,
}
#2 HorizConn {
    begin: Column #6, Line #2,
    length: 2,
    state: Finished,
}
#3 Rect {
    name: "A",
    name_completed: true,
    state: Finished,
    top_left: Column #1, Line #1,
    bottom_right: Some(
        Column #9, Line #8,
    ),
    size: Size {
        height: 4,
        width: 4,
    },
    inputs: [],
    outputs: [],
    collect: "",
}
#4 Rect {
    name: "THING",
    name_completed: true,
    state: Finished,
    top_left: Column #9, Line #0,
    bottom_right: Some(
        Column #29, Line #9,
    ),
    size: Size {
        height: 5,
        width: 10,
    },
    inputs: [],
    outputs: [],
    collect: "",
}
#5 HorizConn {
    begin: Column #40, Line #5,
    length: 4,
    state: Finished,
}
#6 HorizConn {
    begin: Column #28, Line #6,
    length: 1,
    state: Finished,
}
#7 HorizConn {
    begin: Column #14, Line #7,
    length: 2,
    state: Finished,
}
#8 Rect {
    name: "ALMOST",
    name_completed: true,
    state: Finished,
    top_left: Column #30, Line #2,
    bottom_right: Some(
        Column #48, Line #13,
    ),
    size: Size {
        height: 6,
        width: 9,
    },
    inputs: [],
    outputs: [],
    collect: "",
}
#9 Rect {
    name: "CAN",
    name_completed: true,
    state: Finished,
    top_left: Column #17, Line #5,
    bottom_right: Some(
        Column #37, Line #12,
    ),
    size: Size {
        height: 4,
        width: 10,
    },
    inputs: [],
    outputs: [],
    collect: "",
}
#10 Rect {
    name: "THAT",
    name_completed: true,
    state: Finished,
    top_left: Column #0, Line #6,
    bottom_right: Some(
        Column #26, Line #13,
    ),
    size: Size {
        height: 4,
        width: 13,
    },
    inputs: [],
    outputs: [],
    collect: "",
}
#11 HorizConn {
    begin: Column #30, Line #10,
    length: 14,
    state: Finished,
}
#12 HorizConn {
    begin: Column #13, Line #12,
    length: 3,
    state: Finished,
}
#13 HorizConn {
    begin: Column #42, Line #12,
    length: 2,
    state: Finished,
}
#14 HorizConn {
    begin: Column #30, Line #13,
    length: 2,
    state: Finished,
}
#15 Rect {
    name: "PARSE",
    name_completed: true,
    state: Finished,
    top_left: Column #4, Line #11,
    bottom_right: Some(
        Column #20, Line #18,
    ),
    size: Size {
        height: 4,
        width: 8,
    },
    inputs: [],
    outputs: [],
    collect: "",
}
#16 Rect {
    name: "IEC6113-3",
    name_completed: true,
    state: Finished,
    top_left: Column #17, Line #9,
    bottom_right: Some(
        Column #41, Line #20,
    ),
    size: Size {
        height: 6,
        width: 12,
    },
    inputs: [],
    outputs: [],
    collect: "",
}
#17 Rect {
    name: "FBDs",
    name_completed: true,
    state: Finished,
    top_left: Column #33, Line #11,
    bottom_right: Some(
        Column #49, Line #18,
    ),
    size: Size {
        height: 4,
        width: 8,
    },
    inputs: [],
    outputs: [],
    collect: "",
}
#18 Rect {
    name: "SPIDAC",
    name_completed: true,
    state: Finished,
    top_left: Column #45, Line #0,
    bottom_right: Some(
        Column #69, Line #29,
    ),
    size: Size {
        height: 15,
        width: 12,
    },
    inputs: [
        Input {
            name: "CS",
            position: Column #45, Line #1,
        },
        Input {
            name: "SCK",
            position: Column #45, Line #5,
        },
        Input {
            name: "INT1",
            position: Column #45, Line #8,
        },
        Input {
            name: "MOSI",
            position: Column #45, Line #10,
        },
        Input {
            name: "MISO",
            position: Column #45, Line #12,
        },
    ],
    outputs: [
        Output {
            name: "LOUT",
            position: Column #57, Line #5,
        },
        Output {
            name: "ROUT",
            position: Column #57, Line #7,
        },
    ],
    collect: "",
}
```
