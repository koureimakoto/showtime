using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Player : MonoBehaviour
{

    public float speed = 0f;
    public float jump_force;
 
    private Rigidbody2D Rig; 
 
    // Start is called before the first frame update
    void Start() {   
        Rig = GetComponent<Rigidbody2D>();
    }

    // Update is called once per frame
    void Update() {
        Move();
        Jump();
    }


    void Move() {
        float x_axis_input = Input.GetAxis("Horizontal");
        float delta        = Time.deltaTime * speed;
        Vector3 mov        = new Vector3(x_axis_input, 0f, 0f);

        transform.position += mov * delta;
    }

    void Jump() {
        bool y_axis_input = Input.GetButtonDown("Jump");
        if(y_axis_input) {
            Vector2 jump = new Vector2(0f, jump_force);
            Rig.AddForce(
                jump,
                ForceMode2D.Impulse
            );
        }
    }
}
