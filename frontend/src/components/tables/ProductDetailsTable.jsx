// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import React from "react";
import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableRow,
  Paper,
} from "@mui/material";
import { useSelector } from "react-redux";
import { camelCaseToSpaces } from "../../utils";

const ProductDetailsTable = () => {
  const { entities } = useSelector((state) => state.submodels);
  const version = entities.version;

  return (
    <TableContainer component={Paper}>
      <Table sx={{ minWidth: 350 }} aria-label="product details table">
        <TableBody>
          {version && Object.keys(version).map((key) => (
            <TableRow key={key}>
              <TableCell component="th" scope="row">
                {camelCaseToSpaces(key)}
              </TableCell>
              <TableCell align="right">{version[key]}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
};

export default ProductDetailsTable;
