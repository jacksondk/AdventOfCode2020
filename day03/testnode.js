const fs = require("fs");

const data = fs.readFileSync(__dirname + "/input.txt").toString().split("\n").map(row => row.split(""));

const result = [
  //  {rowInc: 1, columnInc: 1},
   // {rowInc: 1, columnInc: 3},
    //{rowInc: 1, columnInc: 5},
    //{rowInc: 1, columnInc: 7},
    {rowInc: 2, columnInc: 1}
]
    .reduce((total, slope) => total * (
                                        data.filter((_row, rowIndex) => rowIndex % slope.rowInc === 0) // Get only used rows
                                            .filter((row, rowIndex) => row[rowIndex * slope.columnInc % data[0].length] === "#") // Get only rows where # is at the right column index
                                            .length
                                    )
, 1)

console.log(result);