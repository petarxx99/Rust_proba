
Pokusavam da ustedim memorijski prostor koji koristim da bi sacuvao podatke o sahovskoj poziciji,
kako bih mogao sto vise pozicija da skladistim tokom analize.

U struct-u Tabla imam 2 niza bajtova za bele i crne figure. Evo kako skladistim informacije u tim 
bitovima.
 Prvih 8 bajtova cuvaju informacije o tome gde se figure nalaze. 
Prvih 6 bitova cuvaju informaciju o tome gde se figura nalaze na tabli. 
Informaciju o tome da li se figura nalazi na tabli cuvam tako sto figure koje su sklonjene sa table
imaju istu lokaciju kao i njihov kralj.
7. i 8. bajt ostaju na raspolaganju pijunu koji se nalazi 8 mesta ispred u nizu. 
7. i 8. bajt cuvaju informaciju o tome u sta se pijun pretvorio (da li je postao kraljica,
top, lovac, konj, itd.). Ako je pijun i dalje pijun, onda 7. i 8. bajt ne sluze ni cemu.
 Sto se tice pijuna, oni se nalaze od 8. do 15. mesta u nizu. Oni koriste prvih 6 bitova za poziciju na tabli,
 7. bit odredjuje da li su promovisani, ili ne, a 8. bit ostaje neiskoriscen. 

Imam i bitfield polje, to je trece polje struct-a Tabla.
U njemu skladistim podatke o tome da li je rokada moguca, da li se pijun pomerio dva polja 
u prethodnom potezu (zbog en passant), ko je na potezu, pre koliko poteza je pojedena figura
ili pomeren pijun (zbog pravila da se posle 50 poteza bez pomeranja pijuna i uzimanja figura 
partija automatski zavrsava nereseno).
U kodu ima komentara i objasnjena koji bitovi su zaduzeni za sta u tom bitfieldu.

U fajlu drvo_eval.rs se nalazi srz sahovskog algoritma. Funkcija izracunaj_rekursivno 
ima logiku koja ce biti srz mog sahovskog algoritma. 
Vec sam u javi napravio sahovsku aplikaciju koja omogucava da 2 igraca igraju izmedju sebe.
Ovaj sahovski program cu na kraju povezati sa tom java aplikacijom.
Za sada je ovaj projekat jos u fazi izrade, kada ce biti zavrsen zavisi od toga koliko budem imao vremena.
U trenutku dok ovo pisem i dalje nisam promenio algoritam koji proverava da li je potez legalan
u java sahovskoj aplikaciji. Kad sam napisao tu aplikaciju nisam odvojio proveru da li 
je potez validan sa odigravanjem poteza, tako da cu sada logiku napisati drugacije, jer je 
sahovskom programu neophodno da nadje sve legalne poteze i izabere najbolji.

Ovo mi je prvi projekat u programskom jeziku Rust. Tek sam krenuo da ucim ovaj programski jezik.


