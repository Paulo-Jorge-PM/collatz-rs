/*$(document).ready(function(){
alert(777)
$("#collatz-result").change(function(){
    alert(888);

    let result = $(this).text();

    alert(result);
    
    var chart = new Chart("graph", {
        type: "scatter",
        data: {
          datasets: [{
            pointRadius: 4,
            pointBackgroundColor: "rgba(0,0,255,1)",
            data: chart
          }]
        },
        options: {}
      });

    
});


});*/

function graph(data) {
    let d = JSON.parse(data);
    new Chart("graph", {
        type: "scatter",
        data: {
          datasets: [{
            pointRadius: 4,
            pointBackgroundColor: "rgb(0,0,255)",
            data: d
          }]
        },
        options: {
          legend: {display: false},
          /*scales: {
            xAxes: [{ticks: {min: 40, max:160}}],
            yAxes: [{ticks: {min: 6, max:16}}],
          }*/
        }
      });
}