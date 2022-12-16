using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Fruit : MonoBehaviour
{
    private SpriteRenderer sprite_render;
    //private SpriteRenderer sprite_collected;
    private BoxCollider2D collider;

    public GameObject collected;
    public GameObject manual_collected;

    // Start is called before the first frame update
    void Start() {
        sprite_render = GetComponent<SpriteRenderer>();
        collider      = GetComponent<BoxCollider2D>();
        //collected     = GetComponent<GameObject>();

        /*
        if(!collected) {
            Debug.Log("Nenhum objeto encontrado");
        }
        
        sprite_collected = collected.GetComponent<SpriteRenderer>();
        if(!!sprite_collected) {
            Debug.Log("Nenhum Sprite Associado");
        }
        */

        manual_collected = new GameObject("collected");
    }

    void DisableObjectAction() {
        sprite_render.enabled = false;
        collider.enabled = false;
    }

    void OnTriggerEnter2D(Collider2D _collider) {
        if(_collider.gameObject.tag == "Player") {
            DisableObjectAction();
            //sprite_collected.enabled = true;
            collected.SetActive(true);

            Destroy(gameObject, .3f);
        }
    }
}
