use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::utils::raw_sql_migration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            UPDATE ARMIES as a set
            name = c.name,
            lore = c.lore
            from (values
              ('Minute Men Militia',
                'The weary yet vigilant militia armies are known for fighting when circumstances turn south and all able-bodied hands are needed to defend territory. These men are essentially peasants with a sword forced into their hands and often fighting against their will. As a result, they''re not the first choice of a general but are available in a pinch.'
            ),('Peacekeeper Monks', 
                'The Kang Lo monks live high up in the mythical floating cities of the far east. They prefer not to fight, but are trained to keep the peace and subdue local vigilantes and invading raiders alike. Sometimes wielding fist weapons, but often fighting with only bracers for defense, rags for clothing and their lightning-fast fists, they''re known for maneuvering and exposing their slower, foes.'
            ),('North Watch Longbowmen', 
                'Arrested vagrants, swindlers, rapists, and murderers have been locked in the open-air prison north of the massive Wall of the Heavens, along with mutants, the contagious, and anyone or anything unfortunate enough to be condemned by the Great Empire. It''s been the responsibility of the highly-paid North Watch Longbowmen to ensure none escape and slip south back into the heart of the empire. Well paid for a reason, the Longbowmen are the best in the realm with the massive longbows made of the ancient yew tree.' 
            ),('Barbarians of the Outer Steppe',
                'The brutal Barbarians of the Outer Steppe are known for their ferociousness and being absolutely thrilled to partake in battle to the point where enemy soldiers have been known to break formation, drop their weapons and run away. These berserkers prefer to use large, two handed war mauls to crush their opponents hiding behind thick armor.'
            ),('Amazonian Huntresses', 
                'Standing two meters tall, these dexterous Huntresses belong to a matriarchal society in the dangerous southern Ta''jun jungle. One misstep can mean the end of life at the hands of  a wide array of creatures or naturally occurring death traps. It''s here that these women have mastered their martial prowess and have been one of the few cultures to repel Great Imperial forces, maintaining their sovereignty before the Great Fall.  They prefer spears at short range and skilled with javelin at a distance.'
            ),('Rōnin Immortals', 
                'Little has been known about the feudal far-east. Tales of hellish beasts, forsaken magic, and the legendary Nephalem percolate back west via the merchant roads but are only half believed. Among these tall tales are the fright-inducing, blood thirsty Immortals who wield katanas in both hands and don a mask imbued with demonic spirits. Immortals they''re called, as they''ve never been witnessed slayed in combat.'
            ),('Castlegate Crossbowmen', 
                'The men who either lack the strength or skill required for the Longbow and aren''t much for swordsmen often end up in the ranks as crossbowmen. Despite some shortcomings, they''re ruthless and self-serving making them very dangerous. Despite not having the range of the longbow, their crossbows are technological marvels that enable high accuracy and a high rate of fire while being less fatiguing on the operator.'
            ),('Oath-Sworn Knights', 
                'Knights have a long history as being protectors of the realm, and they continue to do so even after the fall of the Great Empire. They are unbreakable soldiers dedicated to their sworn oath, and maintain their conviction into battle. Trained since childhood, the Oath-Sworn knights are effective well-rounded swordsmen with equally high defense.'
            ),('Shinobi Martial Artists', 
                'The Shinobi are hooded are vigilantes and criminals with supreme martial arts skills and maneuverability,  wreaking havoc against the merchants, noble class, and any authority who attempts to impose their will. It wasn''t until the fall of the Great Empire that they saw their opportunity to band together with their unique martial prowess in hopes of establishing their own nation within the realm. While adept with edged weapons, their tastes have changed and have adopted concussive nunchuks as their primary armament, maximizing attack speed and chaos in battle.'
            ),('Avian Cliff Dwellers', 
                'The bird-people that call the Central Narrows home have little patience for men passing through their canyons. They will remain concealed  and leap from the cliffs when the opportunity arises, jabbing with their long pikes and looting whatever treasure the wayward travelers were foolish enough to carry with them.'
            ),('Highborn Cavalry', 
                'The Highborn Cavalry are bold, brazen and proud. Born of noblemen and trained by the best in the realm. Mounted on massive pure-bred war steeds, these men charge into battle without hesitation, looking to outperform their fellow cavaliers. Their only priority in war is to be the first in combat, with the most enemies tallied on their lance.'
            ),('Death Dealer Assassins', 
                'The notorious Death Dealer''s guild is an organization that lives in the shadows, holding allegiances with no other organization or nation beyond what they can pay for a particular service. Members are anonymous and any details about what happens within guild meetings is hard to come by. What''s known for certain is these are the most deadly assassins in the world and take their contracts very seriously.'
            ),('Magi Enforcers', 
                'The elite officers that used to protect the inner guard are masters of crowd control and detaining aggressors in the name of ''peace''. They use their short range magic, to cast spells in the fray and bringing even the most heavily armored foes to their knees. They are now elaborately-dressed mercenaries, selling their prowess to the highest bidder.'
            ),('Elven Archers', 
                'The wispy woods of Duskwatch are home to the agile High Elves and typically stay out of human affairs. Since the collapse, the millennia-long treaty held between the elves and the Great Empire is meaningless and they find their people fending off human invaders more frequently than ever. They''ve decided to rejoin the Realm in hopes of securing their land once more. Skillful with the bow and very agile, elven warriors are not to be trifled with.'
            ),('Imperial Legionnaires', 
                'Orderly, obedient and effective, the Legionnaires were the cornerstone of the Imperial armies before the Great Fracture. They were known as being nearly impossible to defeat when fighting within a complete, tightly-packed legion. Standing nearly shoulder to shoulder and protected by their massive tower shields, they will taking with their brethren jabbing with their pilum spears while remaining protected. With the Great Empire fallen, they are no longer committed to any one nation and looking to fight for purpose once more.'
            ),('Skull Clan Death Cultists', 
                'The death cult that''s spread like a contagion through the southern swamp land has been cultivating some of the most devastating death magic known in the realm. Able to cast sinister area of effect magic attacks at a distance strikes fear into most soldiers hearts.  Little is known about where the dark energies are summoned from.'
            )) as c(name, lore)
            WHERE c.name = a.name;
            ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            UPDATE ARMIES
            SET lore = 'TBD'
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
