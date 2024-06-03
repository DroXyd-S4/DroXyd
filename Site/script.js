// Vous pouvez ajouter ici du code JavaScript pour rendre votre site web plus interactif
console.log('Le script JavaScript fonctionne correctement!');


document.getElementById('downloadBtn').addEventListener('click', function() {
    // Crée un élément <a> pour le téléchargement
    var downloadLink = document.createElement('a');
    downloadLink.setAttribute('href', 'Rapport_Soutenance3.pdf'); // Remplacez 'chemin/vers/votre/fichier.pdf' par le chemin de votre fichier à télécharger
    downloadLink.setAttribute('download', 'Soutenance3.pdf'); // Remplacez 'nom-du-fichier.pdf' par le nom que vous souhaitez donner au fichier téléchargé

    // Simule un clic sur le lien pour déclencher le téléchargement
    downloadLink.click();
});