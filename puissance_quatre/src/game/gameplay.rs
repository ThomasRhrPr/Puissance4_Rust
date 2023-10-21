/* Module gameplay.rs
 * 
 * TODO : 
 * - faire un timer
 * - tout commenter
 * - faire de la gestion d'erreur
 * - Simplifier, corriger, donner du sens (surtout d'un aspect modulable)
 * */

use std::io;

use crate::game::grid::Grid;
use crate::game::player::{LocalPlayer, IAPlayer, Player};

pub enum GameMod {
    LocalVsLocal,
    LocalVsIA,
}

pub struct Gameplay
{
    pub grid: Grid,
    pub player1: LocalPlayer,
    pub player2: LocalPlayer,
    pub ia: IAPlayer,
    pub current_player: CurrentPlayer, // Utilisez une référence mutable pour suivre le player actuel
}

#[derive(Copy, Clone)]
pub enum CurrentPlayer
{
    Player1,
    Player2,
}

impl Gameplay
{
    pub fn new_gameplay(grid: Grid, player1: LocalPlayer, player2: LocalPlayer, ia: IAPlayer) -> Self 
    {
        Gameplay 
        {
            grid,
            player1,
            player2,
            ia,
            current_player: CurrentPlayer::Player1, // Initialisez le player actuel avec player1
        }
    }

    pub fn get_player(&self, player: CurrentPlayer) -> &LocalPlayer
    {
        match player {
            CurrentPlayer::Player1 => &self.player1,
            CurrentPlayer::Player2 => &self.player2,
        }

    }

    pub fn get_player_mut(&mut self, player: CurrentPlayer) -> &mut LocalPlayer
    {
        match player {
            CurrentPlayer::Player1 => &mut self.player1,
            CurrentPlayer::Player2 => &mut self.player2,
        }

    }

    pub fn check_line_victory(&self, player_token: char) -> bool 
    {
        // Vérifiez la séquence de tokens dans toutes les lignes
        for ligne in &self.grid.grid 
        {
            let mut count = 0;
            for cellule in ligne.iter() 
            {
                if *cellule == player_token
                {
                    count += 1;
                    if count == 4 
                    {
                        return true; // Victoire détectée dans cette ligne
                    }
                } 
                else 
                {
                    count = 0; // Réinitialisez le compteur si une cellule différente est rencontrée
                }
            }
        }
    
        false // Aucune victoire détectée dans toutes les lignes
    }
    
    pub fn check_column_victory(&self, player_token: char) -> bool 
    {
        let colonnes = self.grid.grid[0].len();
    
        // Parcourez toutes les colonnes pour vérifier s'il y a 4 tokens consécutifs du player spécifié
        for colonne in 0..colonnes 
        {
            let mut count = 0;
            for ligne in &self.grid.grid 
            {
                let cellule = ligne[colonne];
                if cellule == player_token
                {
                    count += 1;
                    if count == 4 
                    {
                        return true; // Victoire détectée dans cette colonne
                    }
                } 
                else 
                {
                    count = 0; // Réinitialisez le compteur si une cellule différente est rencontrée
                }
            }
        }
    
        false // Aucune victoire détectée dans toutes les colonnes
    }

    // Pour la fonction suivante, Mr. Jouault a approuvé l'utilisation des indices pour les itérations
    pub fn check_diagonal_victory(&self, player_token: char) -> bool 
    {
        let lignes = self.grid.grid.len();
        let colonnes = self.grid.grid[0].len();

        // Vérification de haut en bas (de gauche à droite)
        for i in 0..lignes - 3 
        {
            for j in 0..colonnes - 3 
            {
                if self.grid.grid[i][j] == player_token
                    && self.grid.grid[i + 1][j + 1] == player_token
                    && self.grid.grid[i + 2][j + 2] == player_token
                    && self.grid.grid[i + 3][j + 3] == player_token
                {
                    println!("Victoire détectée en diagonale (de gauche à droite) !");
                    return true; // Victoire détectée
                }
            }
        }

        // Vérification de haut en bas (de droite à gauche)
        for i in 0..lignes - 3 
        {
            for j in 3..colonnes 
            {
                if self.grid.grid[i][j] == player_token
                    && self.grid.grid[i + 1][j - 1] == player_token
                    && self.grid.grid[i + 2][j - 2] == player_token
                    && self.grid.grid[i + 3][j - 3] == player_token
                {
                    println!("Victoire détectée en diagonale (de droite à gauche) !");
                    return true; // Victoire détectée
                }
            }
        }

        false
    }

    // Fonction pour vérifier la victoire
    fn check_victory(&self, player_token: char) -> bool 
    {
        // Vous pouvez réutiliser vos fonctions de vérification de victoire ici
        Gameplay::check_line_victory(self, player_token)
            || Gameplay::check_column_victory(self, player_token)
            || Gameplay::check_column_victory(self, player_token)
            || Gameplay::check_diagonal_victory(self, player_token)
    }

    fn choose_mod() -> GameMod
    {
        println!("Choisissez un mode de jeu :");
        println!("Ecrivez 'local' pour une partie Local player vs Local player");
        println!("Ecrivez 'ia' pour une partie Local player vs IA player");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de l'entrée.");
        let choice = input.trim().to_lowercase();
        match choice.as_str()
        {
            "local" => GameMod::LocalVsLocal,
            "ia" => GameMod::LocalVsIA,
            _ => {
                println!("Choix invalide, choisissez à nouveau.");
                Self::choose_mod()
            }
        }

    }

    // Fonction principale pour jouer la partie
    pub fn play(&mut self) 
    {
        self.current_player = CurrentPlayer::Player1;

        let mut tokens_places_player1 = 0;

        let mut game_mod = Self::choose_mod();
        
        loop 
        {
            while !Gameplay::check_victory(&self, self.get_player(self.current_player).get_token()) || !Gameplay::check_victory(&self, self.ia.get_token())
            {
                println!("Tour de {}", self.get_player(self.current_player).name);
                self.grid.display_grid();

                match self.grid.add_token(self.get_player(self.current_player).token, 0) 
                {
                    // L'ajout du token a réussi
                    Ok(_) => 
                    {
                        if self.grid.ask_full() 
                        {
                            println!("Partie terminée. Match nul !");
                            break;
                        }

                        if self.get_player(self.current_player) == &self.player1 
                        {
                            tokens_places_player1 += 1;
                        }
                        if tokens_places_player1 >= 4 
                        {
                            if Gameplay::check_victory(&self, self.get_player(self.current_player).get_token()) 
                            {
                                self.grid.display_grid();
                                println!("Partie terminée. {} a gagné !", self.get_player(self.current_player).name);
                                break;
                            }
                        }

                        match game_mod
                        {
                            GameMod::LocalVsLocal =>
                            {
                                if self.get_player(self.current_player) == &self.player1 
                                {
                                    self.current_player = CurrentPlayer::Player2
                                } else 
                                {
                                    self.current_player = CurrentPlayer::Player1
                                };
                            }

                            GameMod::LocalVsIA =>
                            {
                                match self.grid.add_token(self.ia.get_token(), self.ia.make_random_move()) 
                                {
                                    // L'ajout du token a réussi
                                    Ok(_) => 
                                    {
                                        if self.grid.ask_full() 
                                        {
                                            println!("Partie terminée. Match nul !");
                                            break;
                                        }
                                        if tokens_places_player1 >= 4 
                                        {
                                            if Gameplay::check_victory(&self, self.ia.get_token()) 
                                            {
                                                self.grid.display_grid();
                                                println!("Partie terminée. IA a gagné !");
                                                break;
                                            }
                                        }
                                    }
                                    // L'ajout du token a échoué, affichez un message d'erreur
                                    Err(err) => 
                                    {
                                        println!("Erreur : {}", err);
                                    }
                                }
                            }
                        }
                    }

                    // L'ajout du token a échoué, affichez un message d'erreur
                    Err(err) => 
                    {
                        println!("Erreur : {}", err);
                    }
                }
            }

            // Demandez si les players veulent rejouer
            println!("Voulez-vous rejouer ? (oui/non)");
            let mut input = String::new();

            while (input.trim().to_lowercase() != "oui" && input.trim().to_lowercase() != "non")
            {
                input.clear();
                io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de l'entrée.");
                println!("ici");
            }

            if input.trim().to_lowercase() != "oui" 
            {
                break; // Sortez de la boucle infinie si la réponse n'est pas "oui"
            }

            self.grid.empty_grid();
            tokens_places_player1 = 0;

            game_mod = Self::choose_mod();
        }
    }
}
