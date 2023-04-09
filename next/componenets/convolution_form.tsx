import React, {ChangeEvent, useEffect, useState} from "react";
import {
  Button,
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  SelectChangeEvent,
  Typography,
  Grid,
  TextField, Paper, Card, CardContent, CardActions, Box, Divider
} from "@mui/material";
import {useAppDispatch} from "@/store";

export default function ConvolutionForm() {
  const dispatch = useAppDispatch();
  const [rowNum, setRowNum] = useState(3);
  const [colNum, setColNum] = useState(3);
  const [grid, setGrid] = useState<number[]>(() => {
        const value = 1 / (rowNum * colNum);
        return Array<number>(rowNum * colNum).fill(value);
      }
  );

  useEffect(() => {
    const value = 1 / (rowNum * colNum);
    const newArr = Array<number>(rowNum * colNum).fill(value);
    setGrid(newArr)
  }, [rowNum, colNum])

  const handleRowsChange = (event: SelectChangeEvent<number>) => {
    const newRows = event.target.value as number;
    setRowNum(newRows);
  };

  const handleColsChange = (event: SelectChangeEvent<number>) => {
    const newCols = event.target.value as number;
    setColNum(newCols);
  };

  const handleNumberChange = (event: ChangeEvent<HTMLTextAreaElement | HTMLInputElement>, index: number) => {
    const newVal = parseFloat(event.target.value);

    // deep copy the grid
    const newGrid = [...grid]

    newGrid[index] = newVal;
    setGrid(newGrid);
  };

  const handleButtonClick = () => {
    console.log({grid, rowNum, colNum});
    dispatch({type: "app/addConvolutionOperation", payload: {kernel: grid, width: rowNum, height: colNum}});
  };

  return (
      <Card elevation={3}>
        <CardContent>

          <Grid
              container
              direction="column"
              spacing={3}
          >

            <Grid item container
                  direction="row"
                  spacing={1}

                  justifyContent="space-evenly">
              <Grid item xs>
                <FormControl fullWidth>
                  <InputLabel id="row-select-label">Rows</InputLabel>
                  <Select label="Rows" labelId="row-select-label" value={rowNum}
                          onChange={handleRowsChange}>
                    {[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].map((row) => (
                        <MenuItem key={row} value={row}>{row}</MenuItem>
                    ))}
                  </Select>
                </FormControl>
              </Grid>

              <Grid item xs>
                <FormControl fullWidth>
                  <InputLabel id="col-select-label">Columns</InputLabel>
                  <Select label="Columns" labelId="col-select-label" value={colNum} onChange={handleColsChange}>
                    {[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].map((col) => (
                        <MenuItem key={col} value={col}>{col}</MenuItem>
                    ))}
                  </Select>
                </FormControl>
              </Grid>
            </Grid>


            <Grid item container spacing={1}>
              {grid.map((cell, index) =>
                  <Grid key={index} item xs={12 / colNum}>
                    <TextField onChange={e => handleNumberChange(e, index)}
                               type="number"
                               value={cell}
                               sx={{input: {textAlign: "center"}}}/>
                  </Grid>)}
            </Grid>

          </Grid>
        </CardContent>

        <CardActions>
          <Button variant="outlined" onClick={handleButtonClick}>Add to Pipeline</Button>
        </CardActions>
      </Card>
  );
}