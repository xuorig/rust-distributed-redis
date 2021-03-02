import "./App.css";

import { Pie } from "react-chartjs-2";

function App() {
  const data = {
    datasets: [
      {
        data: [50, 25, 25],
        backgroundColor: [
          "#3e95cd",
          "#8e5ea2",
          "#3cba9f",
          "#e8c3b9",
          "#c45850",
        ],
      },
    ],
    labels: ["A", "B", "C"],
  };
  return (
    <div className="App">
      <Pie data={data} />
    </div>
  );
}

export default App;
