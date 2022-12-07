using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Player : MonoBehaviour
{

    public float speed = 0f;
    // Start is called before the first frame update
    void Start()
    {
        
    }

    // Update is called once per frame
    void Update() {
        Move();
    }


    void Move() {
        float x_axis_input = Input.GetAxis("Horizontal");
        float delta        = Time.deltaTime * speed;
        Vector3 mov        = new Vector3(x_axis_input, 0f, 0f);

        transform.position += mov * delta;
    }
}
