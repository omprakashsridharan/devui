import { createTheme } from '@mui/material/styles';

export const theme = createTheme({
  palette: {
    mode: 'dark',
    primary: { main: '#61dafb' },
    secondary: { main: '#8e9aaf' },
    background: {
      default: '#000000',
      paper: '#0a0a0a',
    },
    text: {
      primary: '#eaeaea',
      secondary: '#a5a5a5',
    },
    divider: '#222222',
  },
  components: {
    MuiCssBaseline: {
      styleOverrides: {
        body: {
          backgroundColor: '#000000',
          color: '#eaeaea',
        },
      },
    },
    MuiPaper: {
      styleOverrides: {
        root: {
          backgroundImage: 'none',
        },
      },
    },
  },
});

export default theme;


