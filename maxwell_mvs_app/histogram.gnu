# set term png
# set output 'gnuplot.png'
set boxwidth 0.9 absolute
set style fill solid
set key fixed right top vertical Right noreverse noenhanced autotitle nobox
set style histogram clustered gap 0.5 title textcolor lt -1
set datafile missing '-'
set style data histograms
set xtics border in scale 0,0 nomirror rotate by -45  autojustify
set xtics  norangelimit 
set xtics   ()
set style line 1 \
    linecolor rgb '#0060ad' \
    linetype 1 linewidth 2 \
    pointtype 7 pointsize 1.5
set title "Maxwell distribution"
# plot 'maxwell.dat' using 2:xtic(1) title "Histogram"
plot 'maxwell.dat' using 2:xtic(1) title "Histogram", \
     'maxwell_pdf.dat' with linespoints linestyle 1 title "PDF"
